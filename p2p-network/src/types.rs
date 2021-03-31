use libp2p::{gossipsub::MessageId, kad::Record};
use serde::{Deserialize, Serialize};
use std::{fmt, time::Duration};

pub type Topic = String;

#[derive(Debug, Clone)]
pub enum Command {
    SubscribeGossipTopic(Topic),
    UnsubscribeGossipTopic(Topic),
    PublishGossipData { data: GossipMessage, topic: Topic },
    GetRecord(String),
    PutRecord { key: String, value: Vec<u8> },
    RemoveRecord(String),
    Shutdown,
}

#[derive(Debug, Clone)]
pub enum CommandResult {
    SubscribeResult(Result<bool, String>),
    UnsubscribResult(Result<bool, String>),
    PublishResult(Result<MessageId, String>),
    GetRecordResult(Result<Vec<Record>, String>),
    PutRecordResult(Result<(), String>),
    RemoveRecordAck,
    ShutdownAck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMessage {
    Message(String),
    SetLed(LedState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LedState {
    On,
    Off,
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
