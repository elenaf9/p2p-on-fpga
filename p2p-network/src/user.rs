use async_std::io;
use core::task::{Context, Poll};
use  futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    prelude::*,
};

use crate::types::*;
use regex::RegexSet;

pub struct PollUser {
    cmd_tx: UnboundedSender<Command>,
    cmd_res_rx: UnboundedReceiver<CommandResult>,
    message_rx: UnboundedReceiver<GossipMessage>,
}

impl PollUser {
    pub fn new(
        cmd_tx: UnboundedSender<Command>,
        cmd_res_rx: UnboundedReceiver<CommandResult>,
        message_rx: UnboundedReceiver<GossipMessage>,
    ) -> Self {
        PollUser {
            cmd_tx,
            cmd_res_rx,
            message_rx,
        }
    }

    pub async fn run(mut self) {
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        loop {
            let command = stdin
                .next()
                .await
                .and_then(|lines| lines.ok())
                .map(Self::parse_input)
                .unwrap_or(Command::Shutdown);
            if let Command::Shutdown = command {
                break;
            }
        }
    }

    async fn subscribe(&mut self, topic: String) {
        let command = Command::SubscribeGossipTopic(topic);
        self.send_channel(&command).await;
        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::SubscribeResult(Ok(true)) => {
                println!("Successfully subscribed");
            }
            CommandResult::SubscribeResult(Ok(false)) => {
                println!("Already subscribed");
            }
            CommandResult::SubscribeResult(Err(err)) => {
                println!("Failed to subscribe{:?}", err);
            }
            _ => unreachable!(),
        }
    }

    async fn unsubscribe(&mut self, topic: String) {
        let command = Command::SubscribeGossipTopic(topic);
        self.send_channel(&command).await;
        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::UnsubscribResult(Ok(true)) => {
                println!("Successfully unsubscribed.");
            }
            CommandResult::UnsubscribResult(Ok(false)) => {
                println!("No aktive subscription to that topic.");
            }
            CommandResult::UnsubscribResult(Err(err)) => {
                println!("Failed to unsubscribe: {:?}.", err);
            }
            _ => unreachable!(),
        }
    }

    async fn publish(&mut self, topic: String, data: GossipMessage) {
        let command = Command::PublishGossipData { data, topic };
        self.send_channel(&command).await;

        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::PublishResult(Ok(message_id)) => {
                println!("Sucessfully published message with id {:?}.", message_id);
            }
            CommandResult::PublishResult(Err(err)) => {
                println!("Failed to publish {:?}.", err);
            }
            _ => unreachable!(),
        }
    }

    async fn get_record(&mut self, key: String) {
        let command = Command::GetRecord(key.clone());
        self.send_channel(&command).await;

        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::GetRecordResult(Ok(data)) => {
                println!("Received Key: {:?}, Value: {:?}.", key, data);
            }
            CommandResult::GetRecordResult(Err(err)) => {
                println!("Failed to get record {:?}.", err);
            }
            _ => unreachable!(),
        }
    }

    async fn put_record(&mut self, key: String, value: Vec<u8>) {
        let command = Command::PutRecord { key, value };
        self.send_channel(&command).await;

        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::PutRecordResult(Ok(data)) => {
                println!("Successfully publihed record.");
            }
            CommandResult::PutRecordResult(Err(err)) => {
                println!("Failed to get record {:?}.", err);
            }
            _ => unreachable!(),
        }
    }

    async fn shutdown(&mut self, key: String, value: Vec<u8>) {
        let command = Command::PutRecord { key, value };
        self.send_channel(&command).await;

        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::PutRecordResult(Ok(data)) => {
                println!("Successfully publihed record.");
            }
            CommandResult::PutRecordResult(Err(err)) => {
                println!("Failed to get record {:?}.", err);
            }
            _ => unreachable!(),
        }
    }

    async fn remove_record(&mut self, key: String) {
        let command = Command::RemoveRecord(key);
        self.send_channel(&command).await;

        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::RemoveRecordAck => {
                println!("Removed Record.");
            }
            _ => unreachable!(),
        }
    }

    async fn send_channel(&mut self, command: &Command) {
        future::poll_fn(|tcx: &mut Context<'_>| match self.cmd_tx.poll_ready(tcx) {
            Poll::Ready(Ok(())) => Poll::Ready(self.cmd_tx.start_send(command.clone())),
            Poll::Ready(err) => Poll::Ready(err),
            _ => Poll::Pending,
        })
        .await
        .expect("Failed to send message to channel.");
    }

    fn parse_input(line: String) -> Command {
        let regex = RegexSet::new(&[
            r"^Subscribe\s+(?P<topic>\w+)$",
            r"^Unsubscribe\s+(?P<topic>\w+)$",
            r"^Publish\s+(?P<topic>\w+)\s+(?P<type>ping|message\s+?P<msg>(\w+))$",
            r"^Get\s+(?P<key>\w+)",
            r"^Put\s+(?P<key>\w+)\s+(?P<value>\w+)",
            r"^Remove\s+(?P<key>\w+)",
            r"^Shutdown$",
        ])
        .expect("Invalid Regex");
        todo!();
    }
}
