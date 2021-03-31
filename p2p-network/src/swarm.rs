use crate::types::*;
mod behaviour;
mod transport;
use behaviour::{Behaviour, BehaviourEvent};
use core::str::FromStr;
use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    prelude::*,
    select,
    task::{Context, Poll},
};
use libp2p::{
    gossipsub::IdentTopic,
    kad::{KademliaEvent, QueryResult},
    swarm::SwarmEvent,
    Multiaddr, Swarm,
};
use transport::TransportLayer;

macro_rules! await_swarm_event {
    ($swarm:expr, $protocol:ident, { $($case:ident$fields:tt $(if $cond:expr)? => $ret:expr),+ }) => {{
        let protocol_name = format!("{}Event", $protocol);
        async {
            loop {
                if let BehaviourEvent::$protocol(event) = $swarm.next().await {
                    match event {
                        $( $protocol_name::$case$fields => $(if $cond)? { return $ret; }, )+
                        _ => {}
                    }
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! chain {
    ($fx:expr $( => |$($a:ident)*| $fy:expr)+) => {
        $fx.map_err(|_| ())$(.and_then(|$($a)*| $fy.map_err(|_|())))+
    }
}

pub struct PollSwarm {
    swarm: Swarm<Behaviour>,
    cmd_rx: UnboundedReceiver<Command>,
    cmd_res_tx: UnboundedSender<CommandResult>,
    message_tx: UnboundedSender<(Topic, GossipMessage)>,
}

impl PollSwarm {
    pub async fn new(
        cmd_rx: UnboundedReceiver<Command>,
        cmd_res_tx: UnboundedSender<CommandResult>,
        message_tx: UnboundedSender<(Topic, GossipMessage)>,
    ) -> Self {
        let transport = TransportLayer::new().unwrap();
        let swarm = Behaviour::build_swarm(transport).await;
        PollSwarm {
            swarm,
            cmd_rx,
            cmd_res_tx,
            message_tx,
        }
    }

    // Start listening to swarm, block thread until a new listener was created or error occured.
    pub async fn start_listening(&mut self) -> Result<Multiaddr, ()> {
        let addr = Multiaddr::from_str("/ip4/0.0.0.0/tcp/0");
        chain!(addr => |a| Swarm::listen_on(&mut self.swarm, a))?;
        loop {
            match self.swarm.next_event().await {
                SwarmEvent::NewListenAddr(addr) => return Ok(addr),
                SwarmEvent::ListenerError { .. } => return Err(()),
                _ => {}
            }
        }
    }

    // // Connect to a peer by their id, fallback to dialing the address.
    // // Block thread until a new listener was created or error occured.
    // pub fn connect_peer(&mut self, target: PeerId, addr: Multiaddr) -> Result<PeerId, ()> {
    //     let dial_peer = Swarm::dial(&mut self.swarm, &target);
    //     chain!( dial_peer => or_else |_e| Swarm::dial_addr(&mut self.swarm, addr.clone()));

    //     await_swarm_event!(self.swarm, {
    //         ConnectionEstablished {peer_id, ..} if peer_id == target => Ok(peer_id),
    //         UnknownPeerUnreachableAddr { address, .. } if address == addr => Err(()),
    //         UnreachableAddr {peer_id, attempts_remaining: 0, ..} if peer_id == target => Err(())
    //     })
    // }

    pub async fn run(mut self) {
        if self.start_listening().await.is_err() {
            return println!("Failed to start listening. Aborting.");
        }
        loop {
            select! {
                user_cmd = self.cmd_rx.next().fuse() => match user_cmd {
                    Some(cmd) => {
                        let res = self.run_command(cmd.clone()).await;
                        if let Err(err) = res {
                            println!("Aborting due to error: {}", err);
                            break;
                        }
                        if let Command::Shutdown = cmd {
                            break;
                        }
                    },
                    None => break
                },
                swarm_event = self.swarm.next_event().fuse() => println!("{:?}", swarm_event),
            };
        }
    }

    async fn run_command(&mut self, cmd: Command) -> Result<(), String> {
        let res = match cmd {
            Command::SubscribeGossipTopic(topic) => {
                let res = self
                    .swarm
                    .behaviour_mut()
                    .subscribe(topic)
                    .map_err(|e| format!("{:?}", e));
                CommandResult::SubscribeResult(res)
            }
            Command::UnsubscribeGossipTopic(topic) => {
                let res = self
                    .swarm
                    .behaviour_mut()
                    .unsubscribe(topic)
                    .map_err(|e| format!("{:?}", e));
                CommandResult::UnsubscribResult(res)
            }
            Command::PublishGossipData { topic, data } => {
                let res = self
                    .swarm
                    .behaviour_mut()
                    .publish_data(topic, &data)
                    .map_err(|e| format!("{:?}", e));
                CommandResult::PublishResult(res)
            }
            Command::GetRecord(key) => {
                let query_id = self.swarm.behaviour_mut().get_record(&key);
                let mut query_result = None;
                while query_result.is_none() {
                    if let BehaviourEvent::Kademlia(KademliaEvent::QueryResult {
                        id,
                        result: QueryResult::GetRecord(get_record_res),
                        ..
                    }) = self.swarm.next().await
                    {
                        if query_id == id {
                            query_result = Some(get_record_res);
                        }
                    }
                }
                let res = query_result
                    .unwrap()
                    .map(|record_ok| {
                        record_ok
                            .records
                            .iter()
                            .map(|peer_rec| peer_rec.record.clone())
                            .collect()
                    })
                    .map_err(|e| format!("{:?}", e));
                CommandResult::GetRecordResult(res)
            }
            Command::PutRecord { key, value } => {
                let res = self
                    .swarm
                    .behaviour_mut()
                    .put_record(&key, value)
                    .map_err(|e| format!("{:?}", e))
                    .and_then(|query_id| {
                        let mut query_result = None;
                        while query_result.is_none() {
                            if let BehaviourEvent::Kademlia(KademliaEvent::QueryResult {
                                id,
                                result: QueryResult::PutRecord(put_record_res),
                                ..
                            }) = self.swarm.next().await
                            {
                                if query_id == id {
                                    query_result = Some(put_record_res);
                                }
                            }
                        }
                        query_result
                            .unwrap()
                            .map(|_| ())
                            .map_err(|e| format!("{:?}", e))
                    });
                CommandResult::PutRecordResult(res)
            }
            Command::Shutdown => CommandResult::ShutdownAck,
            _ => todo!(),
        };
        Self::send_channel(&mut self.cmd_res_tx, res).await
    }

    async fn send_gossip_msg(
        &mut self,
        topic: IdentTopic,
        message: GossipMessage,
    ) -> Result<(), String> {
        // let result = serde_json::from_slice::<GossipMessage>(data);
        let topic = topic.to_string();
        let send = (topic, message);
        Self::send_channel(&mut self.message_tx, send).await
    }

    async fn send_channel<T>(channel: &mut UnboundedSender<T>, message: T) -> Result<(), String> {
        future::poll_fn(|tcx: &mut Context<'_>| match channel.poll_ready(tcx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(err)) => err
                .is_full()
                .then(|| Poll::Pending)
                .unwrap_or(Poll::Ready(Err(err))),
            _ => Poll::Pending,
        })
        .await
        .and_then(|()| channel.start_send(message))
        .map_err(|err| {
            channel.close_channel();
            format!("Forwarding gossip message to channel failed: {:?}", err)
        })
    }
}
