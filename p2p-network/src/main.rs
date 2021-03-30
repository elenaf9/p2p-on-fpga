mod swarm;
mod types;
mod user;
use async_std::task;
use libp2p::futures::{channel::mpsc as channel, join};
use swarm::PollSwarm;
use user::PollUser;
use types::*;

fn main() {
    // Channel for sending commands from user task to swarm
    let (cmd_tx, cmd_rx) = channel::unbounded::<Command>();

    // Channel for returning the outcome of a command
    let (cmd_res_tx, cmd_res_rx) = channel::unbounded::<CommandResult>();

    // Channel for forwarding incoming messages
    let (msg_tx, msg_rx) = channel::unbounded::<GossipMessage>();

    let swarm_handle = task::spawn(async {
        let mut swarm_task = PollSwarm::new(cmd_rx, cmd_res_tx, msg_tx).await;
        swarm_task.run().await
    });
    let input_handle = task::spawn(PollUser::new(cmd_tx, cmd_res_rx, msg_rx).run());

    task::block_on(async { join!(swarm_handle, input_handle) });
}
