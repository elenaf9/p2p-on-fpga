use crate::types::*;
mod behaviour;
mod transport;
use async_std::task;
use behaviour::Behaviour;
use core::str::FromStr;
use libp2p::{
    futures::{
        channel::mpsc::{UnboundedReceiver, UnboundedSender},
        prelude::*,
        select,
    },
    swarm::SwarmEvent,
    Multiaddr, Swarm,
};
use transport::TransportLayer;

macro_rules! await_swarm_event {
    ($swarm:expr, { $($case:ident$fields:tt $(if $cond:expr)? => $ret:expr),+ }) => {
        task::block_on(async {
            loop {
                match $swarm.next_event().await {
                    $( SwarmEvent::$case$fields => $(if $cond)? { return $ret; }, )+
                    _ => {}
                }
            }
        })
    };
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
    message_tx: UnboundedSender<GossipMessage>,
}

impl PollSwarm {
    pub async fn new(
        cmd_rx: UnboundedReceiver<Command>,
        cmd_res_tx: UnboundedSender<CommandResult>,
        message_tx: UnboundedSender<GossipMessage>,
    ) -> Self {
        let transport = TransportLayer::new().unwrap();
        let swarm = Behaviour::build_swarm(transport).await;
        // swarm.behaviour_mut().bootstrap();
        PollSwarm {
            swarm,
            cmd_rx,
            cmd_res_tx,
            message_tx,
        }
    }

    // Start listening to swarm, block thread until a new listener was created or error occured.
    pub fn start_listening(&mut self) -> Result<Multiaddr, ()> {
        let addr = Multiaddr::from_str("/ip4/0.0.0.0/tcp/0");
        chain!(addr => |a| Swarm::listen_on(&mut self.swarm, a))?;

        await_swarm_event!(self.swarm, {
            NewListenAddr(addr) => Ok(addr),
            ListenerError{..} => Err(())
        })
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

    pub async fn poll_futures(mut self) {
        loop {
            select! {
                user_cmd = self.cmd_rx.next().fuse() => match user_cmd {
                    Some(Command::Shutdown) => break,
                    Some(cmd) => self.run_command(cmd),
                    None => break
                },
                swarm_event = self.swarm.next_event().fuse() => println!("{:?}", swarm_event),
            };
        }
    }

    fn run_command(&mut self, cmd: Command) {
        match cmd {
            Command::SubscribeGossipTopic(topic) => {
                self.swarm.behaviour_mut().subscribe(&topic);
            }
            Command::UnsubscribeGossipTopic(topic) => {
                self.swarm.behaviour_mut().unsubscribe(&topic)
            }
            Command::PublishGossipData { topic, data } => {
                let res = self.swarm.behaviour_mut().publish_data(topic, &data);
                if let Err(err) = res {
                    println!("Could not publish data: {:?}", err);
                }
            }
            Command::GetRecord(_id) => {}
            _ => todo!(),
        }
    }
}
