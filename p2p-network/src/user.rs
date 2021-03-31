use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    prelude::*,
    select,
    task::{Context, Poll},
};

use crate::types::*;
use async_std::io::{stdin, BufReader};
use std::{str::FromStr, time::Duration};

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
        let mut stdin = BufReader::new(stdin()).lines();
        let _ = super::cli::build_app().print_long_help();
        println!("\n\n");
        loop {
            select! {
                line = stdin.next().fuse()=> {
                    let command = match line {
                        Some(Ok(line)) => self.parse_input(line),
                        Some(Err(err)) => {
                            println!("Aborting due to error: {}", err);
                            break;
                        }
                        None => {
                            println!("Stdin closed. Aborting.");
                            Some(Command::Shutdown)
                        }
                    };
                    if let Some(command) = command {
                        let res = self.handle_command(command.clone()).await;
                        if let Err(err) = res {
                            println!("Aborting due to error: {}", err);
                            break;
                        }
                        if let Command::Shutdown = command {
                            break;
                        }
                    }
                }
                message = self.message_rx.next().fuse() => match message {
                    Some((topic, message)) => Self::print_incoming(topic, message),
                    None => {
                        println!("Message channel closed unexpected. Aborting.");
                        let _ = self.handle_command(Command::Shutdown).await;
                        break;
                    }
                }
            }
        }
    }

    fn print_incoming(topic: String, message: GossipMessage) {
        match message {
            GossipMessage::Message(msg) => {
                println!("Received gossip message for topic {}:\n{:?}", topic, msg)
            }
            GossipMessage::SetLed(state) => {
                println!("Received command to set Led state: {}", state)
            }
        }
    }

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

    fn parse_input(&mut self, line: String) -> Option<Command> {
        let (args, _) =
            line.split('"')
                .fold((Vec::<String>::new(), 0), |(mut args, index), string| {
                    if index % 2 == 0 {
                        let mut new_args = string
                            .replace("=", " ")
                            .split(' ')
                            .map(|s| s.to_string())
                            .collect();
                        args.append(&mut new_args);
                    } else {
                        args.push(string.to_string())
                    }
                    (args, index + 1)
                });

        let mut app = super::cli::build_app();
        let matches = app
            .get_matches_from_safe_borrow(args)
            .map_err(|_| {
                println!("\nInvalid argument: \"{}\"\n", line);
                let _ = app.print_long_help();
                println!("\n\n");
            })
            .ok()?;

        if let Some(topic) = matches
            .subcommand_matches("subscribe")
            .and_then(|matches| matches.value_of("topic"))
        {
            return Some(Command::SubscribeGossipTopic(topic.to_string()));
        }

        if let Some(topic) = matches
            .subcommand_matches("unsubscribe")
            .and_then(|matches| matches.value_of("topic"))
        {
            return Some(Command::UnsubscribeGossipTopic(topic.to_string()));
        }

        if let Some(topic) = matches
            .subcommand_matches("publish")
            .and_then(|matches| matches.value_of("topic"))
        {
            let topic = topic.to_string();

            if let Some(message) = matches
                .subcommand_matches("message")
                .and_then(|matches| matches.value_of("message"))
            {
                let data = GossipMessage::Message(message.to_string());
                return Some(Command::PublishGossipData { topic, data });
            }

            if let Some(matches) = matches.subcommand_matches("led") {
                let data = if matches.subcommand_matches("on").is_some() {
                    Some(GossipMessage::SetLed(LedState::On))
                } else if matches.subcommand_matches("off").is_some() {
                    Some(GossipMessage::SetLed(LedState::Off))
                } else if let Some(freq) = matches
                    .subcommand_matches("blink")
                    .and_then(|matches| matches.value_of("frequency"))
                    .and_then(|s| u64::from_str(s).ok())
                {
                    Some(GossipMessage::SetLed(LedState::Blink(Duration::from_secs(
                        freq,
                    ))))
                } else {
                    None
                };
                if let Some(data) = data {
                    Some(Command::PublishGossipData { topic, data });
                }
            }
        }

        if let Some(key) = matches
            .subcommand_matches("get-record")
            .and_then(|matches| matches.value_of("key"))
        {
            return Some(Command::GetRecord(key.to_string()));
        }

        if let Some((key, value)) = matches
            .subcommand_matches("put-record")
            .and_then(|matches| matches.value_of("key"))
            .and_then(|k| matches.value_of("value").map(|v| (k.to_string(), v.into())))
        {
            return Some(Command::PutRecord { key, value });
        }

        if let Some(key) = matches
            .subcommand_matches("remove-record")
            .and_then(|matches| matches.value_of("key"))
        {
            return Some(Command::RemoveRecord(key.to_string()));
        }

        if matches.subcommand_matches("shutdown").is_some() {
            return Some(Command::Shutdown);
        }

        None
    }
}
