use async_std::io;
use core::task::{Context, Poll};
use libp2p::futures::{
    channel::mpsc::{SendError, UnboundedSender},
    prelude::*,
};

use super::Commands;

pub struct PollUser {
    tx: UnboundedSender<Commands>,
}

impl PollUser {
    pub fn new(tx: UnboundedSender<Commands>) -> Self {
        PollUser { tx }
    }

    pub async fn poll_user_input(mut self) {
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        loop {
            let command = stdin
                .next()
                .await
                .and_then(|lines| lines.ok())
                .map(Self::parse_input)
                .unwrap_or(Commands::Shutdown);
            if self.send_channel(&command).await.is_err() {
                break;
            } else if let Commands::Shutdown = command {
                break;
            }
        }
    }

    fn parse_input(_line: String) -> Commands {
        todo!();
    }

    async fn send_channel(&mut self, command: &Commands) -> Result<(), SendError> {
        future::poll_fn(|tcx: &mut Context<'_>| match self.tx.poll_ready(tcx) {
            Poll::Ready(Ok(())) => Poll::Ready(self.tx.start_send(command.clone())),
            Poll::Ready(err) => Poll::Ready(err),
            _ => Poll::Pending,
        })
        .await
    }
}
