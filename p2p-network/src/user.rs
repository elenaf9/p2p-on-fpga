use crate::{cli, types::*};
use async_std::io::{self, BufReader};
use futures::{
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
    prelude::*,
    select,
    task::{Context, Poll},
};
use libp2p::Multiaddr;
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
        let mut stdin = BufReader::new(io::stdin()).lines();

        loop {
            // simultainously poll both futures, select the one that return first.
            select! {
                // Poll for input via stdin
                line = stdin.next().fuse()=> {
                    let command = match line {
                        Some(Ok(line)) => self.parse_input(line),
                        Some(Err(err)) => {
                            println!("> Aborting due to error: {}", err);
                            break;
                        }
                        None => {
                            println!("> Stdin closed. Aborting.");
                            Some(Command::Shutdown)
                        }
                    };
                    if let Some(command) = command {
                        let res = self.handle_command(command.clone()).await;
                        if let Err(err) = res {
                            println!("> Aborting due to error: {}", err);
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
                        println!("> Message channel closed unexpected. Aborting.");
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
                println!(
                    "> Received gossip message for topic {}:\n{:?}\n",
                    topic, msg
                )
            }
            GossipMessage::SetLed(state) => {
                println!("> Received command to set led state: {}\n", state)
            }
        }
    }

    // Handle a user command, block task until a result is returned.
    async fn handle_command(&mut self, command: Command) -> Result<(), String> {
        self.send_channel(&command).await?;
        let res = self
            .cmd_res_rx
            .next()
            .await
            .ok_or_else(|| String::from("Channel Error"))?;
        match command {
            Command::SubscribeGossipTopic(..) => self.match_subscribe_res(res),
            Command::UnsubscribeGossipTopic(..) => self.match_unsubscribe_res(res),
            Command::PublishGossipData { .. } => self.match_publish_res(res),
            Command::GetRecord(..) => self.match_get_record_res(res),
            Command::PutRecord { .. } => self.match_put_record_res(res),
            Command::Connect(..) => self.match_connect_res(res),
            Command::Shutdown => self.match_shutdown_res(res),
        };
        Ok(())
    }

    // Print the outcome of the subscribe command.
    fn match_subscribe_res(&mut self, res: CommandResult) {
        match res {
            CommandResult::SubscribeResult(Ok(true)) => {
                println!("> Successfully subscribed\n");
            }
            CommandResult::SubscribeResult(Ok(false)) => {
                println!("> Already subscribeds\n");
            }
            CommandResult::SubscribeResult(Err(err)) => {
                println!("> Failed to subscribe: {:?}s\n", err);
            }
            _ => {}
        }
    }

    // Print the outcome of the unsubscribe command.
    fn match_unsubscribe_res(&mut self, res: CommandResult) {
        match res {
            CommandResult::UnsubscribResult(Ok(true)) => {
                println!("> Successfully unsubscribed.\n");
            }
            CommandResult::UnsubscribResult(Ok(false)) => {
                println!("> No aktive subscription to that topic.\n");
            }
            CommandResult::UnsubscribResult(Err(err)) => {
                println!("> Failed to unsubscribe: {:?}.\n", err);
            }
            _ => {}
        }
    }

    // Print the outcome of the publish command.
    fn match_publish_res(&mut self, res: CommandResult) {
        match res {
            CommandResult::PublishResult(Ok(_)) => {
                println!("> Sucessfully published message.\n");
            }
            CommandResult::PublishResult(Err(err)) => {
                println!("> Failed to publish: {:?}.\n", err);
            }
            _ => {}
        }
    }

    // Print the outcome of the get-record command.
    fn match_get_record_res(&mut self, res: CommandResult) {
        match res {
            CommandResult::GetRecordResult(Ok(vec)) => {
                println!("> Found Record:");
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
            CommandResult::GetRecordResult(Err(error)) => {
                println!("> Failed to get record: {:?}.\n", error);
            }
            _ => {}
        }
    }

    // Print the outcome of the put-record command.
    fn match_put_record_res(&mut self, res: CommandResult) {
        match res {
            CommandResult::PutRecordResult(Ok(())) => {
                println!("> Successfully published record.\n");
            }
            CommandResult::PutRecordResult(Err(err)) => {
                println!("> Failed to get record {:?}.\n", err);
            }
            _ => {}
        }
    }

    // Print the outcome of the connect command.
    fn match_connect_res(&mut self, res: CommandResult) {
        match res {
            CommandResult::ConnectResult(Ok(peer_id)) => {
                println!("> Successfully connected to Peer {}.\n", peer_id);
            }
            CommandResult::ConnectResult(Err(err)) => {
                println!("> Failed to dial the address: {}.\n", err);
            }
            _ => {}
        }
    }

    // Print the outcome of the shutdown command
    fn match_shutdown_res(&mut self, res: CommandResult) {
        if let CommandResult::ShutdownAck = res {
            self.cmd_res_rx.close();
        }
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
                            .filter_map(|s| (!s.is_empty()).then(|| s.to_string()))
                            .collect();
                        args.append(&mut new_args);
                    } else {
                        args.push(string.to_string())
                    }
                    (args, index + 1)
                });

        // Use the command line interface to parse the users arguments.
        let mut app = cli::build_app();

        // Checks if the line matches the required input, then try for each subsommand if it matches.
        let matches = app
            .get_matches_from_safe_borrow(args.clone())
            .map_err(|_| {
                let mut out = Vec::new();
                let (is_sub, mut help) = match line {
                    _ if args.contains(&"subscribe".to_string()) => (true, cli::subscribe_cmd()),
                    _ if args.contains(&"unsubscribe".to_string()) => {
                        (true, cli::unsubscribe_cmd())
                    }
                    _ if args.contains(&"publish".to_string()) => (true, cli::publish_cmd()),
                    _ if args.contains(&"get-record".to_string()) => (true, cli::get_record_cmd()),
                    _ if args.contains(&"put-record".to_string()) => (true, cli::put_record_cmd()),
                    _ if args.contains(&"connect".to_string()) => (true, cli::connect_cmd()),
                    _ => (false, app),
                };
                let subcommand_string = is_sub.then(|| "\n p2p SUBCOMMAND \n").unwrap_or("\n");
                help.write_long_help(&mut out)
                    .expect("Failed to write long help message");
                let message = String::from_utf8(out).expect("Invalid help-message string");
                println!(
                    "\n> Invalid argument: \"{}\"\n---------------{}---------------\n{}\n",
                    line, subcommand_string, message
                );
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

        if let Some(addr_string) = matches
            .subcommand_matches("connect")
            .and_then(|matches| matches.value_of("address"))
        {
            if let Ok(addr) = Multiaddr::from_str(addr_string) {
                return Some(Command::Connect(addr));
            } else {
                println!("> Failed to parse the given address into a Multiaddress.\n");
            }
        }

        if matches.subcommand_matches("shutdown").is_some() {
            return Some(Command::Shutdown);
        }

        None
    }
}
