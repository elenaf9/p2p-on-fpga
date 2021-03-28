use libp2p::{
    gossipsub::{
        error::{PublishError, SubscriptionError},
        IdentTopic, MessageId,
    },
    kad::{GetRecordError, PutRecordError, Record},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Command {
    SubscribeGossipTopic(IdentTopic),
    UnsubscribeGossipTopic(IdentTopic),
    PublishGossipData {
        data: GossipMessage,
        topic: IdentTopic,
    },
    GetRecord(String),
    PutRecord {
        key: String,
        value: Vec<u8>,
    },
    RemoveRecord(String),
    Shutdown,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CommandResult {
    SubscribeResult(Result<bool, SubscriptionError>),
    UnsubscribResult(Result<bool, PublishError>),
    PublishResult(Result<MessageId, PublishError>),
    GetRecordResult(Result<Vec<Record>, GetRecordError>),
    PutRecordResult(Result<(), PutRecordError>),
    RemoveRecordAck,
    ShutdownAck,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GossipMessage {
    Ping,
    Message(String),
}
