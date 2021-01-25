
use crate::behaviour::msg_protocol::{MessageCodec, MessageProtocol, Request, Response};
use async_std::{
    io,
    net::{Shutdown, TcpListener, TcpStream},
    task,
};
use serde::Deserialize;

#[test]
fn send_request() {
    let listener = task::block_on(async { TcpListener::bind("127.0.0.1:8081").await.unwrap() });
    let listener_handle = task::spawn(async move {
        let mut incoming = listener.incoming();
        let stream = incoming.next().await.unwrap().unwrap();
        let (reader, writer) = &mut (&stream, &stream);
        io::copy(reader, writer).await.unwrap();
    });

    let writer_handle = task::spawn(async {
        let protocol = MessageProtocol;
        let mut codec = MessageCodec;
        let mut socket = TcpStream::connect("127.0.0.1:8081").await.unwrap();
        let message = Request::Message(String::from("test request"));
        codec
            .write_request(&protocol, &mut socket, message.clone())
            .await
            .unwrap();
        let received = codec.read_request(&protocol, &mut socket).await.unwrap();
        socket.shutdown(Shutdown::Both).unwrap();
        assert_eq!(message, received);
    });
    task::block_on(async {
        listener_handle.await;
        writer_handle.await;
    })
}

#[test]
fn send_response() {
    let listener = task::block_on(async { TcpListener::bind("127.0.0.1:8082").await.unwrap() });
    let listener_handle = task::spawn(async move {
        let mut incoming = listener.incoming();
        let stream = incoming.next().await.unwrap().unwrap();
        let (reader, writer) = &mut (&stream, &stream);
        io::copy(reader, writer).await.unwrap();
    });

    let writer_handle = task::spawn(async {
        let protocol = MessageProtocol;
        let mut codec = MessageCodec;
        let mut socket = TcpStream::connect("127.0.0.1:8082").await.unwrap();
        let message = Response::Message(String::from("test response"));
        codec
            .write_response(&protocol, &mut socket, message.clone())
            .await
            .unwrap();
        let received = codec.read_response(&protocol, &mut socket).await.unwrap();
        socket.shutdown(Shutdown::Both).unwrap();
        assert_eq!(message, received);
    });
    task::block_on(async {
        listener_handle.await;
        writer_handle.await;
    })
}
