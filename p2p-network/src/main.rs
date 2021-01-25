pub mod behaviour;
mod transport;
use async_std::{io, task};
use behaviour::{Behaviour, Request};
use core::task::{Context, Poll};
use libp2p::{
    futures::{
        channel::mpsc::{self as channel, UnboundedReceiver, UnboundedSender},
        join,
        prelude::*,
        select,
    },
    core::ConnectedPoint,
    swarm::SwarmEvent,
    Multiaddr, PeerId, Swarm,
};
use transport::TransportLayer;

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Commands {
    ConnectPeer { peer_id: PeerId, addr: Multiaddr },
    SendRequest { request: Request, peer_id: PeerId },
    SubscribeGossipTopic(String),
    PublishGossipData { data: Vec<u8>, topic: String },
    GetRecord(String),
    PutRecord { key: String, value: Vec<u8> },
    RemoveRecord(String),
}

struct PollSwarm {
    swarm: Swarm<Behaviour>,
    rx: UnboundedReceiver<Commands>,
}

impl PollSwarm {
    pub async fn new(rx: UnboundedReceiver<Commands>) -> Self {
        let transport = TransportLayer::new().unwrap();
        let swarm = Behaviour::build_swarm(transport).await;
        PollSwarm { swarm, rx }
    }

    pub fn start_listening(&mut self) -> Result<Multiaddr, ()> {
        Swarm::listen_on(&mut self.swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).map_err(|_| ())?;
        task::block_on(async {
            loop {
                match self.swarm.next_event().await {
                    SwarmEvent::NewListenAddr(addr) => return Ok(addr),
                    SwarmEvent::ListenerError { error: _ } => return Err(()),
                    _ => {}
                }
            }
        })
    }

    pub fn connect_peer(
        &mut self,
        target_peer_id: PeerId,
        addr: Multiaddr,
    ) -> Result<(PeerId, Multiaddr), ()> {
        if Swarm::dial(&mut self.swarm, &target_peer_id).is_err()
            && Swarm::dial_addr(&mut self.swarm, addr.clone()).is_err()
        {
            return Err(());
        }
        // block until the connection either failed 
        return task::block_on(async {
            loop {
                match self.swarm.next_event().await {
                    SwarmEvent::ConnectionEstablished {
                        peer_id,
                        endpoint: ConnectedPoint::Dialer{address},
                        num_established: _,
                    } => {
                        if peer_id == target_peer_id {
                            return Ok((peer_id, address));
                        }
                    }
                    SwarmEvent::UnknownPeerUnreachableAddr { address, error: _ } => {
                        if address == addr {
                            return Err(());
                        }
                    }
                    SwarmEvent::UnreachableAddr {
                        peer_id,
                        address: _,
                        error: _,
                        attempts_remaining: 0,
                    } => {
                        if peer_id == target_peer_id {
                            return Err(());
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    pub async fn poll_futures(mut self) {
        loop {
            select! {
                user_cmd = self.rx.next().fuse() =>  if let Some(cmd) = user_cmd {
                        self.run_command(cmd)
                    } else {
                        return
                    },
                swarm_event = self.swarm.next_event().fuse() => println!("{:?}", swarm_event),
            };
        }
    }

    fn run_command(&mut self, cmd: Commands) {
        match cmd {
            Commands::SendRequest { request, peer_id } => {
                self.swarm.send_request(&peer_id, request);
            }
            Commands::ConnectPeer { peer_id, addr } => {
                self.connect_peer(peer_id, addr).unwrap();
            }
            _ => todo!(),
        }
    }
}

struct PollUser {
    tx: UnboundedSender<Commands>,
}

impl PollUser {
    pub fn new(tx: UnboundedSender<Commands>) -> Self {
        PollUser { tx }
    }

    pub async fn poll_user_input(mut self) {
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        let s = &mut self;
        loop {
            let command = select! {
                stdin_input = stdin.next().fuse() => {
                    if let Some(Ok(line)) = stdin_input {
                        Some(s.parse_input(line))
                    } else {
                        None
                    }
                },
                peripheral_input = PollUser::poll_peripherals().fuse() => peripheral_input,
            };
            if let Some(command) = command {
                task::block_on(future::poll_fn(|tcx: &mut Context<'_>| {
                    match s.tx.poll_ready(tcx) {
                        Poll::Ready(Ok(())) => Poll::Ready(s.tx.start_send(command.clone())),
                        Poll::Ready(err) => Poll::Ready(err),
                        _ => Poll::Pending,
                    }
                }))
                .unwrap();
            }
        }
    }

    async fn poll_peripherals() -> Option<Commands> {
        todo!();
    }

    fn parse_input(&mut self, _line: String) -> Commands {
        todo!();
    }
}

fn main() {
    let (tx, rx) = channel::unbounded();
    let mut swarm_task = task::block_on(PollSwarm::new(rx));
    swarm_task.start_listening().unwrap();
    let swarm_handle = task::spawn(swarm_task.poll_futures());
    let input_handle = task::spawn(PollUser::new(tx).poll_user_input());
    task::block_on(async { join!(swarm_handle, input_handle) });
}
