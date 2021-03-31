mod swarm;
mod types;
mod user;
use futures::{channel::mpsc as channel, executor, join};
use swarm::PollSwarm;
use types::*;
use user::PollUser;

fn main() {
    // Channel for sending commands from user task to swarm
    let (cmd_tx, cmd_rx) = channel::unbounded::<Command>();

    // Channel for returning the outcome of a command
    let (cmd_res_tx, cmd_res_rx) = channel::unbounded::<CommandResult>();

    // Channel for forwarding incoming messages
    let (msg_tx, msg_rx) = channel::unbounded::<(Topic, GossipMessage)>();

    // Start a future for polling the swarm and managing swarm interaction.
    let swarm_handle = async {
        let swarm_task = PollSwarm::new(cmd_rx, cmd_res_tx, msg_tx).await;
        swarm_task.run().await
    };

    // Start a future for polling user input and deciding how to handle messages.
    let input_handle = PollUser::new(cmd_tx, cmd_res_rx, msg_rx).run();

    // Poll both futures simultaneously untill both returned.
    executor::block_on(async { join!(swarm_handle, input_handle) });
}
