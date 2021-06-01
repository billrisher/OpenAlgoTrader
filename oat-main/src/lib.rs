pub mod myserver;
pub mod client;
pub mod pubsub;

use std::thread;

fn main() {
    let mut processes = vec![];

    processes.push(thread::spawn(myserver::myserver::run_server));
    processes.push(thread::spawn(client::client::run_client));
    pubsub::pubsub::pubsub();

    // runs until child threads stop running
    for proc in processes {
        let _ = proc.join();
    }
}
