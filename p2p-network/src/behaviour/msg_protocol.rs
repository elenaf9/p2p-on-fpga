use async_trait::async_trait;
use core::fmt::Debug;
use libp2p::futures::{prelude::*, AsyncRead, AsyncWrite};
use libp2p::{
    core::{
        upgrade::{read_one, write_one},
        ProtocolName,
    },
    request_response::RequestResponseCodec,
};
use serde::{Deserialize, Serialize};
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};

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

#[derive(Debug, Clone, PartialEq)]
pub struct MessageProtocol;

impl ProtocolName for MessageProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/p2p/1".as_bytes()
    }
}

#[derive(Clone)]
pub struct MessageCodec;

#[async_trait]
impl RequestResponseCodec for MessageCodec {
    type Protocol = MessageProtocol;
    type Request = Request;
    type Response = Response;

    // read requests from remote peers and parse them into the request struct
    async fn read_request<R>(&mut self, _: &MessageProtocol, io: &mut R) -> IOResult<Self::Request>
    where
        R: AsyncRead + Unpin + Send,
    {
        read_one(io, usize::MAX)
            .map(|req| match req {
                Ok(bytes) => serde_json::from_slice(bytes.as_slice())
                    .map_err(|e| IOError::new(IOErrorKind::InvalidData, e)),
                Err(e) => Err(IOError::new(IOErrorKind::InvalidData, e)),
            })
            .await
    }

    // read responses from remote peers and parse them into the request struct
    async fn read_response<R>(
        &mut self,
        _: &MessageProtocol,
        io: &mut R,
    ) -> IOResult<Self::Response>
    where
        R: AsyncRead + Unpin + Send,
    {
        read_one(io, usize::MAX)
            .map(|res| match res {
                Ok(bytes) => serde_json::from_slice(bytes.as_slice())
                    .map_err(|e| IOError::new(IOErrorKind::InvalidData, e)),
                Err(e) => Err(IOError::new(IOErrorKind::InvalidData, e)),
            })
            .await
    }

    // deserialize request and write to the io socket
    async fn write_request<R>(
        &mut self,
        _: &MessageProtocol,
        io: &mut R,
        req: Self::Request,
    ) -> IOResult<()>
    where
        R: AsyncWrite + Unpin + Send,
    {
        let buf =
            serde_json::to_vec(&req).map_err(|e| IOError::new(IOErrorKind::InvalidData, e))?;
        write_one(io, buf).await
    }

    // deserialize response and write to the io socket
    async fn write_response<R>(
        &mut self,
        _: &MessageProtocol,
        io: &mut R,
        res: Self::Response,
    ) -> IOResult<()>
    where
        R: AsyncWrite + Unpin + Send,
    {
        let buf =
            serde_json::to_vec(&res).map_err(|e| IOError::new(IOErrorKind::InvalidData, e))?;
        write_one(io, buf).await
    }
}
