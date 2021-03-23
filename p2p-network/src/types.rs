use libp2p::{Multiaddr, PeerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Commands {
    ConnectPeer { peer_id: PeerId, addr: Multiaddr },
    SendRequest { request: Request, peer_id: PeerId },
    SubscribeGossipTopic(String),
    PublishGossipData { data: Vec<u8>, topic: String },
    GetRecord(String),
    PutRecord { key: String, value: Vec<u8> },
    RemoveRecord(String),
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LedState {
    On,
    Off,
    Blink(u16),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcedureRequest {
    GetValue { key: String },
    Store { key: String, value: Vec<u8> },
    SetLedState { pin: u32, state: LedState },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcedureResult {
    Success,
    Failure(String),
    Record { key: String, value: Vec<u8> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Request {
    Ping,
    Message(String),
    Procedure(ProcedureRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Response {
    Pong,
    Message(String),
    Procedure(ProcedureResult),
}
