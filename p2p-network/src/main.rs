mod swarm;
mod types;
mod user;
use async_std::task;
use libp2p::futures::{channel::mpsc as channel, join};
use swarm::PollSwarm;
use user::PollUser;

fn main() {
    // channel for sending commands from user task to swarm
    let (cmd_tx, cmd_rx) = channel::unbounded();

    // channel for responding with the outcome of a command
    let (cmd_res_tx, cmd_res_rx) = channel::unbounded();

    // channel for forwarding incoming messages from the swarm
    let (msg_tx, msg_rx) = channel::unbounded();

    let swarm_handle = task::spawn(async {
        let mut swarm_task = PollSwarm::new(cmd_rx, cmd_res_tx, msg_tx).await;
        swarm_task
            .start_listening()
            .expect("Failed to start listening.");
        swarm_task.poll_futures().await
    });
    let input_handle = task::spawn(PollUser::new(cmd_tx, cmd_res_rx, msg_rx).poll_user_input());

    task::block_on(async { join!(swarm_handle, input_handle) });
}
