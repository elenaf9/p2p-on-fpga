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
        record::{store::MemoryStore, Key as RecordKey},
        store::Error as StoreError,
        Kademlia, KademliaEvent, QueryId, Quorum, Record,
    },
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{NetworkBehaviourAction, NetworkBehaviourEventProcess, PollParameters, Swarm},
    NetworkBehaviour,
};

// Out-event that may be returned when polling the Behaviour.
// Created from Kademlia or Gossibsub event that emerged in the Swarm.
#[derive(Debug)]
pub enum BehaviourEvent {
    Kademlia(KademliaEvent),
    Gossipsub(GossipsubEvent),
}

// Create a Network Behaviou structure that combines the protocols mdns, kademlia and gossibsub.
// Based on the Behaviour a swarm is created, as entrypoint for all network interaction.
// Polling the Swarm for events returns a libp2p::swarm::SwarmEvent, in case of a Gossipsub or
// Kademlia event, the respective `BehaviourEvent` for it is issued.
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
    // Create a new Behaviour use it together with the provided transport to build a Swarm.
    // The behaviour methods can be accessed from a swarm struct via Swarm.behaviour,
    // additionally the methods from libp2p::Swarm for dialing, listening, etc. can be used.
    pub async fn build_swarm(transport: TransportLayer) -> Swarm<Behaviour> {
        // Create the network behaviour
        let behaviour = Behaviour::new(&transport)
            .await
            .expect("Failed to create Network Behaviour.");

        // Set the local peer id to match the transport layer, i.g. the public key used
        // for authentication on the transport.
        let peer_id = transport.local_peer_id();

        // Create a swarm.
        Swarm::new(transport.build().await, behaviour, peer_id)
    }

    // Subscribe to a gossipsub topic
    pub fn subscribe(&mut self, topic: String) -> Result<bool, SubscriptionError> {
        let topic = IdentTopic::new(topic);
        self.gossipsub.subscribe(&topic)
    }

    // Unsubscribe from a gossipsub topic
    pub fn unsubscribe(&mut self, topic: String) -> Result<bool, PublishError> {
        let topic = IdentTopic::new(topic);
        self.gossipsub.unsubscribe(&topic)
    }

    // Publish data to a gossipsub topic
    pub fn publish_data(
        &mut self,
        topic: String,
        data: &GossipMessage,
    ) -> Result<MessageId, PublishError> {
        let topic = IdentTopic::new(topic);
        // Serialize the the GossipMessage struct into a byte vector.
        let data_vec = serde_json::to_vec(data).expect("Could not serialize data.");
        self.gossipsub.publish(topic, data_vec)
    }

    // Initiate a kademlia query for a record.
    pub fn get_record(&mut self, key: String) -> QueryId {
        let key = RecordKey::new(&key);
        self.kademlia.get_record(&key, Quorum::One)
    }

    // Initiate a kademlia query to publish a record in the DHT.
    pub fn put_record(&mut self, key: String, value: Vec<u8>) -> Result<QueryId, StoreError> {
        let key = RecordKey::new(&key);
        let record = Record::new(key, value);
        self.kademlia.put_record(record, Quorum::One)
    }

    // Create a new Behaviour with mdns, kademlia and gossibsub protocols.
    // The Behaviour itself is only used in the context of a swarm, that is created with the
    // build_swarm method.
    async fn new(transport: &TransportLayer) -> Result<Behaviour, ()> {
        // Create mDNS protocol
        let mdns = Mdns::new(MdnsConfig::default()).await.map_err(|_| ())?;

        // Create kademlia protocol with an in-memory storing of records.
        let kademlia = {
            let store = MemoryStore::new(transport.local_peer_id());
            Kademlia::new(transport.local_peer_id(), store)
        };

        // Create gossipsub protocol with default config, sign messages with the same keypair
        // that is used to build the transport layer
        let gossipsub = {
            let gossipsub_config = GossipsubConfigBuilder::default().build().unwrap();
            Gossipsub::new(
                MessageAuthenticity::Signed(transport.keypair().clone()),
                gossipsub_config,
            )
            .unwrap()
        };

        // Create and return new behaviour with the protocols.
        Ok(Behaviour {
            mdns,
            kademlia,
            gossipsub,
            is_bootstrapped: false,
            events: Vec::new(),
        })
    }

    // Method that is called when the swarm is polled.
    // If it returns Poll::Ready(event), the event is returned when polling swarm.next().
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

// Handle event from the mDNS protocol.
// The mDNS event is freqently issued by the mDNS protocol, and return the list of peers and have
// been discovered or expired within the last period.
impl NetworkBehaviourEventProcess<MdnsEvent> for Behaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            // Add discovered peers and addresses to kademlia routing table
            for (peer_id, multiaddr) in list {
                self.kademlia.add_address(&peer_id, multiaddr);
            }
            // Bootstrap kademlia if the first peer is discovered.
            // The bootrapping process introduces the local peer to the kademlia DHT by adding
            // the local peers information to the routing table of the closest peers, and adding
            // their information to the local routing table.
            if !self.is_bootstrapped {
                self.is_bootstrapped = self.kademlia.bootstrap().is_ok();
            }
        }
    }
}

// Handle Kademlia event by adding it to the local events, which results in it being returned
// when the Behaviour.poll method is called
impl NetworkBehaviourEventProcess<KademliaEvent> for Behaviour {
    fn inject_event(&mut self, event: KademliaEvent) {
        self.events.push(BehaviourEvent::Kademlia(event));
    }
}

// Handle Gossipsub event by adding it to the local events, which results in it being returned
// when the Behaviour.poll method is called.
impl NetworkBehaviourEventProcess<GossipsubEvent> for Behaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        self.events.push(BehaviourEvent::Gossipsub(event));
    }
}
