mod swarm;
mod types;
mod user;
use async_std::task;
use libp2p::futures::{channel::mpsc as channel, join};
use swarm::PollSwarm;
use types::Commands;
use user::PollUser;

fn main() {
    let (tx, rx) = channel::unbounded();
    let swarm_handle = task::spawn(async {
        let mut swarm_task = PollSwarm::new(rx).await;
        swarm_task
            .start_listening()
            .expect("Failed to start listening.");
        swarm_task.poll_futures().await
    });
    let input_handle = task::spawn(PollUser::new(tx).poll_user_input());

    task::block_on(async { join!(swarm_handle, input_handle) });
}
