use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    prelude::*,
    task::{Context, Poll},
};

use crate::types::*;
use regex::RegexSet;
use std::{
    io::{self, BufRead},
    thread,
    time::Duration,
};

pub struct PollUser {
    cmd_tx: UnboundedSender<Command>,
    cmd_res_rx: UnboundedReceiver<CommandResult>,
    message_rx: UnboundedReceiver<(Topic, GossipMessage)>,
}

impl PollUser {
    pub fn new(
        cmd_tx: UnboundedSender<Command>,
        cmd_res_rx: UnboundedReceiver<CommandResult>,
        message_rx: UnboundedReceiver<(Topic, GossipMessage)>,
    ) -> Self {
        PollUser {
            cmd_tx,
            cmd_res_rx,
            message_rx,
        }
    }

    pub async fn run(mut self) {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        loop {
            match lines.next() {
                Some(line) => {
                    let command = line.map(Self::parse_input).unwrap_or(Command::Shutdown);
                    let res = self.handle_command(command.clone()).await;
                    if let Err(err) = res {
                        println!("Aborting due to error: {}", err);
                        break;
                    }
                    if let Command::Shutdown = command {
                        break;
                    }
                }
                None => match self.message_rx.try_next() {
                    Ok(Some((topic, message))) => match message {
                        GossipMessage::Message(msg) => {
                            println!("Received gossip message for topic {}:\n{:?}", topic, msg)
                        }
                        GossipMessage::SetLed(state) => {
                            println!("Received command to set Led state: {}", state)
                        }
                    },
                    Ok(None) => {}
                    Err(err) => {
                        println!("Aborting due to error: {}", err);
                        break;
                    }
                },
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    async fn print_incoming() {}

    async fn handle_command(&mut self, command: Command) -> Result<(), String> {
        match command {
            Command::SubscribeGossipTopic(topic) => self.subscribe(topic).await,
            Command::UnsubscribeGossipTopic(topic) => self.unsubscribe(topic).await,
            Command::PublishGossipData { topic, data } => self.publish(topic, data).await,
            Command::GetRecord(key) => self.get_record(key).await,
            Command::PutRecord { key, value } => self.put_record(key, value).await,
            Command::RemoveRecord(key) => self.remove_record(key).await,
            Command::Shutdown => self.shutdown().await,
        }
    }

    async fn subscribe(&mut self, topic: String) -> Result<(), String> {
        let command = Command::SubscribeGossipTopic(topic);
        self.send_channel(&command).await?;
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
        Ok(())
    }

    async fn unsubscribe(&mut self, topic: String) -> Result<(), String> {
        let command = Command::SubscribeGossipTopic(topic);
        self.send_channel(&command).await?;
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
        Ok(())
    }

    async fn publish(&mut self, topic: String, data: GossipMessage) -> Result<(), String> {
        let command = Command::PublishGossipData { data, topic };
        self.send_channel(&command).await?;

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
        Ok(())
    }

    async fn get_record(&mut self, key: String) -> Result<(), String> {
        let command = Command::GetRecord(key.clone());
        self.send_channel(&command).await?;

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
        Ok(())
    }

    async fn put_record(&mut self, key: String, value: Vec<u8>) -> Result<(), String> {
        let command = Command::PutRecord { key, value };
        self.send_channel(&command).await?;

        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::PutRecordResult(Ok(())) => {
                println!("Successfully publihed record.");
            }
            CommandResult::PutRecordResult(Err(err)) => {
                println!("Failed to get record {:?}.", err);
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    async fn remove_record(&mut self, key: String) -> Result<(), String> {
        let command = Command::RemoveRecord(key);
        self.send_channel(&command).await?;

        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::RemoveRecordAck => {
                println!("Removed Record.");
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), String> {
        let command = Command::Shutdown;
        self.send_channel(&command).await?;

        let res = self.cmd_res_rx.next().await;
        if let CommandResult::ShutdownAck = res.expect("Channel error") {
            self.cmd_res_rx.close();
        }
        Ok(())
    }

    async fn send_channel(&mut self, command: &Command) -> Result<(), String> {
        future::poll_fn(|tcx: &mut Context<'_>| match self.cmd_tx.poll_ready(tcx) {
            Poll::Ready(Ok(())) => Poll::Ready(self.cmd_tx.start_send(command.clone())),
            Poll::Ready(err) => Poll::Ready(err),
            _ => Poll::Pending,
        })
        .await
        .map_err(|err| format!("Error in channel for sending command: {:?}", err))
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
