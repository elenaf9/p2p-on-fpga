use libp2p::{
    gossipsub::{
        error::{PublishError, SubscriptionError},
        IdentTopic, MessageId,
    },
    kad::{GetRecordError, PutRecordError, Record},
};
use serde::{Deserialize, Serialize};
use core::time::Duration;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Command {
    SubscribeGossipTopic(String),
    UnsubscribeGossipTopic(String),
    PublishGossipData {
        data: GossipMessage,
        topic: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMessage {
    Message(String),
    SetLed(LedState)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LedState {
    On,
    Off,
    Blink(Duration)
}
