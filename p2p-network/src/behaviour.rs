mod msg_protocol;
use crate::transport::TransportLayer;
use core::{iter, time::Duration};
use libp2p::{
    gossipsub::{
        Gossipsub, GossipsubConfigBuilder, GossipsubEvent, IdentTopic, MessageAuthenticity,
    },
    kad::{record::store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{Mdns, MdnsEvent},
    request_response::{
        ProtocolSupport, RequestId, RequestResponse, RequestResponseConfig, RequestResponseEvent,
    },
    swarm::{NetworkBehaviourEventProcess, Swarm},
    NetworkBehaviour, PeerId,
};
use msg_protocol::{MessageCodec, MessageProtocol};
pub use msg_protocol::{Request, Response};

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    mdns: Mdns,
    kademlia: Kademlia<MemoryStore>,
    gossipsub: Gossipsub,
    reqres: RequestResponse<MessageCodec>,
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Behaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                self.kademlia.add_address(&peer_id, multiaddr);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<KademliaEvent> for Behaviour {
    fn inject_event(&mut self, _event: KademliaEvent) {
        todo!()
    }
}

impl NetworkBehaviourEventProcess<RequestResponseEvent<Request, Response>> for Behaviour {
    fn inject_event(&mut self, _event: RequestResponseEvent<Request, Response>) {
        todo!()
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for Behaviour {
    fn inject_event(&mut self, _event: GossipsubEvent) {
        todo!()
    }
}

impl Behaviour {
    async fn new(transport: &TransportLayer) -> Result<Behaviour, ()> {
        let mdns = Mdns::new().await.map_err(|_| ())?;
        let kademlia = {
            let store = MemoryStore::new(transport.local_peer_id());
            Kademlia::new(transport.local_peer_id(), store)
        };
        let gossipsub = {
            let gossipsub_config = GossipsubConfigBuilder::default().build().unwrap();
            Gossipsub::new(
                MessageAuthenticity::Signed(transport.keypair().clone()),
                gossipsub_config,
            )
            .unwrap()
        };
        let reqres = {
            let mut cfg = RequestResponseConfig::default();
            cfg.set_connection_keep_alive(Duration::from_secs(60));
            let protocols = iter::once((MessageProtocol, ProtocolSupport::Full));
            RequestResponse::new(MessageCodec, protocols, cfg)
        };
        Ok(Behaviour {
            mdns,
            kademlia,
            gossipsub,
            reqres,
        })
    }

    pub async fn build_swarm(transport: TransportLayer) -> Swarm<Behaviour> {
        let behaviour = Behaviour::new(&transport).await.unwrap();
        let peer_id = transport.local_peer_id();
        Swarm::new(transport.build(), behaviour, peer_id)
    }

    pub fn subscribe(&mut self, topic: &str) {
        let topic = IdentTopic::new(topic);
        self.gossipsub.subscribe(&topic).unwrap();
    }

    pub fn send_request(&mut self, peer_id: &PeerId, request: Request) -> RequestId {
        self.reqres.send_request(peer_id, request)
    }
}
