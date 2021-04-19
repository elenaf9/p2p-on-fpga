use crate::types::*;
mod behaviour;
mod transport;
use async_std::task::{self, Context, Poll};
use behaviour::{Behaviour, BehaviourEvent};
use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    prelude::*,
    select,
};
use libp2p::{
    gossipsub::{error::PublishError, GossipsubEvent, GossipsubMessage},
    kad::{GetRecordError, GetRecordOk, KademliaEvent, PutRecordOk, QueryId, QueryResult},
    swarm::SwarmEvent,
    Multiaddr, Swarm,
};
use std::str::FromStr;
use transport::TransportLayer;

// Task to manage all swarm interaction and polling.
pub struct SwarmTask {
    // The swarm that serves as entry-point for all network interaction.
    swarm: Swarm<Behaviour>,
    // Channel to receive commands from the user.
    cmd_rx: UnboundedReceiver<Command>,
    // Channel to return the outcome of a command to the user
    cmd_res_tx: UnboundedSender<CommandResult>,
    // Channel to forward gossibsub message that are received in the network.
    message_tx: UnboundedSender<(Topic, GossipMessage)>,
}

impl SwarmTask {
    // Create a new instance of a swarm task
    pub async fn new(
        cmd_rx: UnboundedReceiver<Command>,
        cmd_res_tx: UnboundedSender<CommandResult>,
        message_tx: UnboundedSender<(Topic, GossipMessage)>,
    ) -> Self {
        // Create transport layer
        let transport = TransportLayer::new();

        // Build Swarm based on the transport and behaviour protocols/
        let swarm = Behaviour::build_swarm(transport).await;

        SwarmTask {
            swarm,
            cmd_rx,
            cmd_res_tx,
            message_tx,
        }
    }

    // Start listening to swarm, block thread until a new listener was created or error occured.
    pub async fn start_listening(&mut self) -> Result<Multiaddr, ()> {
        // Listen to a multiaddress assigned by the OS.
        Multiaddr::from_str("/ip4/0.0.0.0/tcp/0")
            .map_err(|_| ())
            .and_then(|a| Swarm::listen_on(&mut self.swarm, a).map_err(|_| ()))?;

        // Poll swarm until either the swarm starts listening, or an listener error occurs.
        // On Success return the actual, OS assigned, listening address.
        loop {
            match self.swarm.next_event().await {
                SwarmEvent::NewListenAddr(addr) => return Ok(addr),
                SwarmEvent::ListenerError { .. } => return Err(()),
                _ => {}
            }
        }
    }

    // Kick off the swarm task in a future (asynchronous operation)
    pub async fn run(mut self) {
        // Start listening to the swarm for incoming requests and queries from the network
        if self.start_listening().await.is_err() {
            return println!("Failed to start listening. Aborting.");
        }

        println!("\n\nLocal peer Id: {:?}\n", self.swarm.local_peer_id());
        loop {
            // Simultainously poll both futures, select the one that return first.
            select! {
                // Command received via the channel from the user task.
                user_cmd = self.cmd_rx.next().fuse() => {
                    match user_cmd {
                    Some(cmd) => {
                        // Handle the received command
                        let res = self.run_command(cmd.clone()).await;
                        // Abort on Error.
                        if let Err(err) = res {
                            println!("Aborting due to error: {}", err);
                            break;
                        }
                        // Break loop/return on shutdown command
                        if let Command::Shutdown = cmd {
                            break;
                        }
                    },
                    None => break
                }},
                // BehaviourEvent that occured in the Swarm.
                // swarm.next() only returns BehaviourEvents from gossibsub and kademlia
                // swarm.next_event() returns all libp2p::swarm::SwarmeEvents, which includes apart from
                // SwarmEvent::Behaviour(BehaviourEvent) also the swarm events for e.g. listening, connection established, ...
                event = self.swarm.next().fuse() => {
                    if let BehaviourEvent::Gossipsub(GossipsubEvent::Message {
                        message: GossipsubMessage { data, topic, .. },
                        ..
                    }) = event {
                        // Try to deserialize the received data back into the GossipMessage that it was serialzed from.
                        if let Ok(msg) = serde_json::from_slice::<GossipMessage>(&data) {
                            // Send message via channel to user task.
                            let send = self.send_gossip_msg(topic.into_string(), msg).await;
                            if send.is_err() {
                                break;
                            }
                        }
                    }
                }
            };
        }
    }

    // Await the query result for a kademlia query to get or publish a record in the DHT.
    fn await_query_result<T>(
        &mut self,
        query_id: QueryId,
        f: &dyn Fn(&QueryResult) -> Option<T>,
    ) -> Result<T, String> {
        // Block current thread until a result for the query was received.
        task::block_on(async {
            loop {
                // Await next behaviour event
                match self.swarm.next().await {
                    BehaviourEvent::Kademlia(KademliaEvent::QueryResult { id, result, .. }) => {
                        // Return if result was for the query was received.
                        let is_query = id == query_id;
                        if let Some(value) = is_query.then(|| f(&result)).flatten() {
                            return Ok(value);
                        }
                    }
                    BehaviourEvent::Gossipsub(GossipsubEvent::Message {
                        message: GossipsubMessage { data, topic, .. },
                        ..
                    }) => {
                        // Try parse and send received gossipsub message to user task.
                        if let Ok(msg) = serde_json::from_slice::<GossipMessage>(&data) {
                            let send = self.send_gossip_msg(topic.into_string(), msg).await;
                            if let Err(err) = send {
                                return Err(err);
                            }
                        }
                    }
                    _ => {}
                }
            }
        })
    }

    // Execute the command recieved from the user task.
    // With swarm.behaviour_mut(), the methods of the underlying Behaviour struct
    // of the swarm are accessed.
    // Return a result or acknowledgment for each command.
    // For Gossibesub events, the result can directly be returned/
    // In case of kademlia queries, the swarm has to be polled for a Kademlia event
    // that signales the outcome of the query, via the await_query_result method.
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
                    .map_err(|e| match e {
                        PublishError::InsufficientPeers => {
                            "No known peers are subscribing to that topic.".into()
                        }
                        _ => format!("{:?}", e),
                    });
                CommandResult::PublishResult(res)
            }
            Command::GetRecord(key) => {
                // Initiate kademlia query for a record.
                let query_id = self.swarm.behaviour_mut().get_record(key);

                // determine what query result matches the issued kademlia query
                let is_match = |event: &QueryResult| match event {
                    QueryResult::GetRecord(Ok(GetRecordOk { records, .. })) => {
                        let records = records
                            .iter()
                            .map(|peer_rec| peer_rec.record.clone())
                            .collect();
                        Some(Ok(records))
                    }
                    QueryResult::GetRecord(Err(GetRecordError::NotFound { key, .. })) => {
                        let e = String::from_utf8(key.to_vec()).unwrap_or(format!("{:?}", key));
                        Some(Err(GetRecordErr::NotFound(e)))
                    }
                    QueryResult::GetRecord(Err(e)) => {
                        Some(Err(GetRecordErr::Other(format!("{:?}", e))))
                    }
                    _ => None,
                };

                // Poll swarm until a matching query result is returned.
                let res = self.await_query_result(query_id, &is_match)?;
                CommandResult::GetRecordResult(res)
            }
            Command::PutRecord { key, value } => {
                // Initiate kademlia query to publish a record.
                // This queries the peer who's id is closest to the hash of the record key to store
                // the record. Fails if that peer fails to store it.
                let put_result = self.swarm.behaviour_mut().put_record(key, value);
                let res = match put_result {
                    Ok(query_id) => {
                        // Determine what query result matches the issued kademlia query
                        let is_match = |event: &QueryResult| match event {
                            QueryResult::PutRecord(Ok(PutRecordOk { .. })) => Some(Ok(())),
                            QueryResult::PutRecord(Err(e)) => Some(Err(format!("{:?}", e))),
                            _ => None,
                        };

                        // Poll swarm until a matching query result is returned.
                        self.await_query_result(query_id, &is_match)?
                    }
                    Err(err) => Err(format!("{:?}", err)),
                };
                CommandResult::PutRecordResult(res)
            }
            Command::Shutdown => CommandResult::ShutdownAck,
        };
        Self::send_channel(&mut self.cmd_res_tx, res).await
    }

    // Forward a gossipsub message via the channel to the user task.
    async fn send_gossip_msg(
        &mut self,
        topic: String,
        message: GossipMessage,
    ) -> Result<(), String> {
        let send = (topic, message);
        Self::send_channel(&mut self.message_tx, send).await
    }

    // Poll the channel until it is ready to send at least one message, then send the message.
    // Failes if the channel is closed.
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
