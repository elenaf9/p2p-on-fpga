use super::Commands;
mod behaviour;
mod msg_protocol;
mod transport;
use async_std::task;
use behaviour::Behaviour;
use core::str::FromStr;
use libp2p::{
    futures::{channel::mpsc::UnboundedReceiver, prelude::*, select},
    swarm::SwarmEvent,
    Multiaddr, PeerId, Swarm,
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

macro_rules! chain {
    ($fx:expr $( => $chain:ident |$($a:ident)*| $fy:expr)+) => {
        $fx.map_err(|_| ())$(.$chain(|$($a)*| $fy.map_err(|_|())))+?
    }
}

pub struct PollSwarm {
    swarm: Swarm<Behaviour>,
    rx: UnboundedReceiver<Commands>,
}

impl PollSwarm {
    pub async fn new(rx: UnboundedReceiver<Commands>) -> Self {
        let transport = TransportLayer::new().unwrap();
        let swarm = Behaviour::build_swarm(transport).await;
        PollSwarm { swarm, rx }
    }

    // Start listening to swarm, block thread until a new listener was created or error occured.
    pub fn start_listening(&mut self) -> Result<Multiaddr, ()> {
        let addr = Multiaddr::from_str("/ip4/0.0.0.0/tcp/0");
        chain!(addr => and_then |a| Swarm::listen_on(&mut self.swarm, a));

        await_swarm_event!(self.swarm, {
            NewListenAddr(addr) => Ok(addr),
            ListenerError{..} => Err(())
        })
    }

    // Connect to a peer by their id, fallback to dialing the address.
    // Block thread until a new listener was created or error occured.
    pub fn connect_peer(&mut self, target: PeerId, addr: Multiaddr) -> Result<PeerId, ()> {
        let dial_peer = Swarm::dial(&mut self.swarm, &target);
        chain!( dial_peer => or_else |_e| Swarm::dial_addr(&mut self.swarm, addr.clone()));

        await_swarm_event!(self.swarm, {
            ConnectionEstablished {peer_id, ..} if peer_id == target => Ok(peer_id),
            UnknownPeerUnreachableAddr { address, .. } if address == addr => Err(()),
            UnreachableAddr {peer_id, attempts_remaining: 0, ..} if peer_id == target => Err(())
        })
    }

    pub async fn poll_futures(mut self) {
        loop {
            select! {
                user_cmd = self.rx.next().fuse() => match user_cmd {
                    Some(Commands::Shutdown) => break,
                    Some(cmd) => self.run_command(cmd),
                    None => break
                },
                swarm_event = self.swarm.next_event().fuse() => println!("{:?}", swarm_event),
            };
        }
    }

    fn run_command(&mut self, cmd: Commands) {
        match cmd {
            Commands::SendRequest { peer_id: _, request: _ } => {
                todo!();
                // self.swarm.send_request(&peer_id, request);
            }
            Commands::ConnectPeer { peer_id, addr } => {
                self.connect_peer(peer_id, addr).unwrap();
            }
            _ => todo!(),
        }
    }
}
