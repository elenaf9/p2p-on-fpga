use super::transport::TransportLayer;
use crate::types::*;
use libp2p::{
    gossipsub::{
        error::PublishError, Gossipsub, GossipsubConfigBuilder, GossipsubEvent, GossipsubMessage,
        IdentTopic, MessageAuthenticity, MessageId,
    },
    kad::{record::store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess, Swarm},
    NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    mdns: Mdns,
    kademlia: Kademlia<MemoryStore>,
    gossipsub: Gossipsub,
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

impl NetworkBehaviourEventProcess<GossipsubEvent> for Behaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        if let GossipsubEvent::Message {
            message_id,
            message:
                GossipsubMessage {
                    source,
                    data,
                    topic,
                    ..
                },
            ..
        } = event
        {
            let source = source
                .map(|p| p.to_base58())
                .unwrap_or_else(|| String::from("Anonymous"));
            println!(
                "Received Message {}\nTopic: {}\nPublished by {}\nData:{:?}",
                message_id,
                topic.into_string(),
                source,
                data
            )
        }
    }
}

impl Behaviour {
    pub async fn build_swarm(transport: TransportLayer) -> Swarm<Behaviour> {
        let behaviour = Behaviour::new(&transport)
            .await
            .expect("Failed to create Network Behaviour.");
        let peer_id = transport.local_peer_id();
        Swarm::new(transport.build().await, behaviour, peer_id)
    }

    pub fn subscribe(&mut self, topic: &IdentTopic) -> bool {
        self.gossipsub
            .subscribe(topic)
            .expect("Failed to subscribe.")
    }

    pub fn unsubscribe(&mut self, topic: &IdentTopic) {
        self.gossipsub
            .unsubscribe(topic)
            .expect("Failed to unsubscribe.");
    }

    pub fn publish_data(
        &mut self,
        topic: IdentTopic,
        data: &GossipMessage,
    ) -> Result<MessageId, PublishError> {
        let data_vec = serde_json::to_vec(data).expect("Could not serialize data.");
        self.gossipsub.publish(topic, data_vec)
    }

    async fn new(transport: &TransportLayer) -> Result<Behaviour, ()> {
        let mdns = Mdns::new(MdnsConfig::default()).await.map_err(|_| ())?;
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
        Ok(Behaviour {
            mdns,
            kademlia,
            gossipsub,
        })
    }
}
