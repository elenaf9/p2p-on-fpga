use libp2p::{gossipsub::MessageId, kad::Record, Multiaddr, PeerId};
use serde::{Deserialize, Serialize};
use std::{fmt, time::Duration};

pub type Topic = String;

// Command that is created based on user input and transferred to the swarm task.
// This Command represents a kademlia or gossibsub operation.
#[derive(Debug, Clone)]
pub enum Command {
    // Explicitely connect a peer by address.
    Connect(Multiaddr),
    // Subscribe to a gossibsub topic so that all messages published to that topic
    // will be received.
    SubscribeGossipTopic(Topic),
    // Unsubscribe from a gossibsub topic so that no more messages for that topic
    /// are received.
    UnsubscribeGossipTopic(Topic),
    // Publish a gossipmessage to a topic.
    // All Peers that are subscribed to that topic will received the message.
    // If no peer subscribing to that topic is known, it will fail with
    // SubscriptionError::PublishError(PublishError::InsufficientPeers)
    PublishGossipData {
        data: GossipMessage,
        topic: Topic,
    },
    // Query for a kademlia record that has to be published to the DHT.
    GetRecord(String),
    // Publish a record to the DHT so that any peer can query for it.
    // This will query the peer closest to the record key (using XOR metric)
    // to store the record.
    PutRecord {
        key: String,
        value: Vec<u8>,
    },
    // Shutdown the swarm task.
    Shutdown,
}

#[derive(Debug, Clone)]
pub enum CommandResult {
    // Result of the attempt to connect a peer by address.
    ConnectResult(Result<PeerId, String>),
    // Result of the subscribe command.
    // Return Ok(true) if successfully subscribe,
    // Ok(false) if the peer is already subscribed to the topic.
    SubscribeResult(Result<bool, String>),
    // Result of the unsubscribe command.
    // Return Ok(true) if successfully unsubscribed
    // Ok(false) if the peer was not subscribed to the topic
    UnsubscribResult(Result<bool, String>),
    // Result for publishing a message to a gossipsub topic.
    PublishResult(Result<MessageId, String>),
    // Result of querying the DHT for a record.
    // Can return multiple records if they use the same key.
    GetRecordResult(Result<Vec<Record>, String>),
    // Result of publishing a record to the DHT
    PutRecordResult(Result<(), String>),
    // Acknowledge shutdown command
    ShutdownAck,
}

// Example for Gossibsub Messages that could be published to certain topics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMessage {
    // Plain message String
    Message(String),
    // Instruct to set the user-LED on the device (if there is one)
    SetLed(LedState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LedState {
    On,
    Off,
    // Frequently blink LED
    Blink(Duration),
}

impl fmt::Display for LedState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LedState::On => write!(f, "on"),
            LedState::Off => write!(f, "off"),
            LedState::Blink(duration) => write!(f, "blink every {}s", duration.as_secs()),
        }
    }
}
