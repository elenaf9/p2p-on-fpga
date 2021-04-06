use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    prelude::*,
    select,
    task::{Context, Poll},
};

use crate::types::*;
use async_std::io::{stdin, BufReader};
use std::{str::FromStr, time::Duration};

// Task that handles all user and periphery interaction
pub struct UserTask {
    // Channel to send commands to swarm task
    cmd_tx: UnboundedSender<Command>,
    // Channel that the swarm task uses to return the results for a command
    cmd_res_rx: UnboundedReceiver<CommandResult>,
    // Channel for incoming gossibsub messages that are received in the network.
    message_rx: UnboundedReceiver<(Topic, GossipMessage)>,
}

impl UserTask {
    // Create new instance of a User Task
    pub fn new(
        cmd_tx: UnboundedSender<Command>,
        cmd_res_rx: UnboundedReceiver<CommandResult>,
        message_rx: UnboundedReceiver<(Topic, GossipMessage)>,
    ) -> Self {
        let _ = super::cli::build_app().print_long_help();
        UserTask {
            cmd_tx,
            cmd_res_rx,
            message_rx,
        }
    }

    // Future (asynchonour Operation) that polls stdin and the message_rx channel
    // for user input and incoming messages that are forwarded from the swarm task
    pub async fn run(mut self) {
        // Read from standard input
        let mut stdin = BufReader::new(stdin()).lines();

        loop {
            // simultainously poll both futures, select the one that return first.
            select! {
                // Poll for input via stdin
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
                // Poll for incoming gossipsub messages
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

    // Print to standard output the gossipsub message that was received
    fn print_incoming(topic: String, message: GossipMessage) {
        match message {
            GossipMessage::Message(msg) => {
                println!("Received gossip message for topic {}:\n{:?}", topic, msg)
            }
            GossipMessage::SetLed(state) => {
                println!("Received command to set led state: {}", state)
            }
        }
    }

    // Handle a user command
    async fn handle_command(&mut self, command: Command) -> Result<(), String> {
        match command {
            Command::SubscribeGossipTopic(topic) => self.subscribe(topic).await,
            Command::UnsubscribeGossipTopic(topic) => self.unsubscribe(topic).await,
            Command::PublishGossipData { topic, data } => self.publish(topic, data).await,
            Command::GetRecord(key) => self.get_record(key).await,
            Command::PutRecord { key, value } => self.put_record(key, value).await,
            Command::Shutdown => self.shutdown().await,
        }
    }

    // Send command to subscribe to a gossipsub topic to swarm task.
    // Poll Command-Result channel for result.
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

    // Send command to unsubscribe froma gossipsub topic to swarm task.
    // Poll Command-Result channel for result.
    async fn unsubscribe(&mut self, topic: String) -> Result<(), String> {
        let command = Command::UnsubscribeGossipTopic(topic);
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

    // Send command to publish a gossipsub message for a topic to swarm task.
    // Poll Command-Result channel for result.
    async fn publish(&mut self, topic: String, data: GossipMessage) -> Result<(), String> {
        let command = Command::PublishGossipData { data, topic };
        self.send_channel(&command).await?;

        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::PublishResult(Ok(_)) => {
                println!("Sucessfully published message with.");
            }
            CommandResult::PublishResult(Err(err)) => {
                println!("Failed to publish: {:?}.", err);
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    // Send command to query for a kademlia record to swarm task.
    // Poll Command-Result channel for result.
    async fn get_record(&mut self, key: String) -> Result<(), String> {
        let command = Command::GetRecord(key.clone());
        self.send_channel(&command).await?;

        let res = self.cmd_res_rx.next().await;
        match res.expect("Channel error") {
            CommandResult::GetRecordResult(Ok(vec)) => {
                println!("Found Records:");
                for record in vec {
                    if let Ok(message) = String::from_utf8(record.value.to_vec()) {
                        let pub_str = record
                            .publisher
                            .map(|p| format! {",\n\tpublisher: {:?}", p})
                            .unwrap_or_else(String::new);
                        println!("\t{:?},\n\tValue: {:?}{}.\n", record.key, message, pub_str);
                    }
                }
            }
            CommandResult::GetRecordResult(Err(GetRecordErr::NotFound(key))) => {
                println!("No record with key {:?} was found.", key);
            }
            CommandResult::GetRecordResult(Err(GetRecordErr::Other(err))) => {
                println!("Failed to get record {:?}.", err);
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    // Send command to publish a kademlia record to swarm task.
    // Poll Command-Result channel for result.
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

    // Send shutdown command to swarm task, poll for result.
    async fn shutdown(&mut self) -> Result<(), String> {
        let command = Command::Shutdown;
        self.send_channel(&command).await?;

        let res = self.cmd_res_rx.next().await;
        if let CommandResult::ShutdownAck = res.expect("Channel error") {
            self.cmd_res_rx.close();
        }
        Ok(())
    }

    // Send a command via the channel to the swarm Task.
    // Fails if the channel is full or closed.
    async fn send_channel(&mut self, command: &Command) -> Result<(), String> {
        future::poll_fn(|tcx: &mut Context<'_>| match self.cmd_tx.poll_ready(tcx) {
            Poll::Ready(Ok(())) => Poll::Ready(self.cmd_tx.start_send(command.clone())),
            Poll::Ready(err) => Poll::Ready(err),
            _ => Poll::Pending,
        })
        .await
        .map_err(|err| format!("Error in channel for sending command: {:?}", err))
    }

    // Parse an users input line to the respective Command
    fn parse_input(&mut self, line: String) -> Option<Command> {
        // Split line into the arguments
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

        // Use the command line interface to parse the users arguments.
        let mut app = super::cli::build_app();

        // Checks if the line matches the required input, then try for each subsommand if it matches.
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

        if let Some((topic, matches)) = matches
            .subcommand_matches("publish")
            .and_then(|matches| matches.value_of("topic").map(|t| (t, matches)))
        {
            let topic = topic.to_string();

            if let Some(value) = matches
                .subcommand_matches("message")
                .and_then(|matches| matches.value_of("value"))
            {
                let data = GossipMessage::Message(value.to_string());
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
                    return Some(Command::PublishGossipData { topic, data });
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
            .and_then(|matches| matches.value_of("key").map(|k| (k, matches)))
            .and_then(|(k, matches)| matches.value_of("value").map(|v| (k.to_string(), v.into())))
        {
            return Some(Command::PutRecord { key, value });
        }

        if matches.subcommand_matches("shutdown").is_some() {
            return Some(Command::Shutdown);
        }

        None
    }
}
