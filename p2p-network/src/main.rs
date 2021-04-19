//! # Basic Peer-to-Peer network using the libp2p concepts and protocols.
//!
//! ## USAGE
//!
//! Start two peers in different terminal windows:
//!
//! ```sh
//! $ cargo run
//!
//! p2p 0.1.0
//! Elena Frank
//! CLI for the p2p-network interaction
//!
//! USAGE:
//!     p2p [SUBCOMMAND]
//!
//! SUBCOMMANDS:
//!     get-record     query for a kademlia record
//!     publish        publish data to certain gossip-sub topic
//!     put-record     publish a record to the kademlia DHT
//!     shutdown       shutdown the app
//!     subscribe      subscribe to a gossip-sub topic
//!     unsubscribe    unsubscribe from a gossip-sub topic
//!
//! Local peer Id: PeerId("12D3KooWBJTyq5sNop5PSkw5Yvc56t3HGgVQ27SYrwUvYLY42EW4")
//! ```
//! Run different commands, e.g.:
//! ```sh
//! p2p subscribe -t my_topic
//! ```
//!
//! Publishing a message to a topic requires that at least one other peer exists that is subscribing to that topic.
//!

mod cli;
mod swarm;
mod types;
mod user;
use async_std::task;
use futures::{channel::mpsc as channel, join};
use swarm::SwarmTask;
use types::*;
use user::UserTask;

fn main() {
    // Channel for sending commands from user task to swarm task
    let (cmd_tx, cmd_rx) = channel::unbounded::<Command>();

    // Channel for returning the outcome of a command
    let (cmd_res_tx, cmd_res_rx) = channel::unbounded::<CommandResult>();

    // Channel for forwarding incoming messages from remote peers
    let (msg_tx, msg_rx) = channel::unbounded::<(Topic, GossipMessage)>();

    // Start a future for polling user input and deciding how to handle messages.
    let input_handle = task::spawn(async {
        let user_task = UserTask::new(cmd_tx, cmd_res_rx, msg_rx);
        user_task.run().await
    });

    // Start a future for polling the swarm and managing swarm interaction.
    let swarm_handle = task::spawn(async {
        let swarm_task = SwarmTask::new(cmd_rx, cmd_res_tx, msg_tx).await;
        swarm_task.run().await
    });

    // Poll both futures simultaneously untill both returned.
    task::block_on(async { join!(input_handle, swarm_handle) });
}
