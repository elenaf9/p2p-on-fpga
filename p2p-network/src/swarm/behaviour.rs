use super::transport::TransportLayer;
use crate::types::*;
use async_std::task::{Context, Poll};
use libp2p::{
    gossipsub::{
        error::{PublishError, SubscriptionError},
        Gossipsub, GossipsubConfigBuilder, GossipsubEvent, IdentTopic, MessageAuthenticity,
        MessageId,
    },
    kad::{
        record::{store::MemoryStore, Key},
        store::Error as StoreError,
        Kademlia, KademliaEvent, QueryId, Quorum, Record,
    },
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{NetworkBehaviourAction, NetworkBehaviourEventProcess, PollParameters, Swarm},
    NetworkBehaviour,
};

#[derive(Debug)]
pub enum BehaviourEvent {
    Kademlia(KademliaEvent),
    Gossipsub(GossipsubEvent),
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BehaviourEvent", poll_method = "poll")]
pub struct Behaviour {
    mdns: Mdns,
    kademlia: Kademlia<MemoryStore>,
    gossipsub: Gossipsub,
    #[behaviour(ignore)]
    is_bootstrapped: bool,
    #[behaviour(ignore)]
    events: Vec<BehaviourEvent>,
}

impl Behaviour {
    pub async fn build_swarm(transport: TransportLayer) -> Swarm<Behaviour> {
        let behaviour = Behaviour::new(&transport)
            .await
            .expect("Failed to create Network Behaviour.");
        let peer_id = transport.local_peer_id();
        Swarm::new(transport.build().await, behaviour, peer_id)
    }

    pub fn subscribe(&mut self, topic: String) -> Result<bool, SubscriptionError> {
        let topic = IdentTopic::new(topic);
        self.gossipsub.subscribe(&topic)
    }

    pub fn unsubscribe(&mut self, topic: String) -> Result<bool, PublishError> {
        let topic = IdentTopic::new(topic);
        self.gossipsub.unsubscribe(&topic)
    }

    pub fn publish_data(
        &mut self,
        topic: String,
        data: &GossipMessage,
    ) -> Result<MessageId, PublishError> {
        let topic = IdentTopic::new(topic);
        let data_vec = serde_json::to_vec(data).expect("Could not serialize data.");
        self.gossipsub.publish(topic, data_vec)
    }

    pub fn get_record(&mut self, key: String) -> QueryId {
        let key = Key::new(&key);
        self.kademlia.get_record(&key, Quorum::Majority)
    }

    pub fn put_record(&mut self, key: String, value: Vec<u8>) -> Result<QueryId, StoreError> {
        let key = Key::new(&key);
        let record = Record::new(key, value);
        self.kademlia.put_record(record, Quorum::Majority)
    }

    pub fn remove_record(&mut self, key: String) {
        let key = Key::new(&key);
        self.kademlia.remove_record(&key)
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
            is_bootstrapped: false,
            events: Vec::new(),
        })
    }

    fn poll<TEv>(
        &mut self,
        _cx: &mut Context<'_>,
        _params: &mut impl PollParameters,
    ) -> Poll<NetworkBehaviourAction<TEv, BehaviourEvent>> {
        if !self.events.is_empty() {
            return Poll::Ready(NetworkBehaviourAction::GenerateEvent(self.events.remove(0)));
        }
        Poll::Pending
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Behaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                self.kademlia.add_address(&peer_id, multiaddr);
            }
            if !self.is_bootstrapped {
                self.is_bootstrapped = self.kademlia.bootstrap().is_ok();
            }
        }
    }
}

impl NetworkBehaviourEventProcess<KademliaEvent> for Behaviour {
    fn inject_event(&mut self, event: KademliaEvent) {
        self.events.push(BehaviourEvent::Kademlia(event));
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for Behaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        self.events.push(BehaviourEvent::Gossipsub(event));
    }
}
