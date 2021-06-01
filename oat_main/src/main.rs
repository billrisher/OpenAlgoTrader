extern crate oat_lib;

use oat_lib::module_client::client::run_client;
use oat_lib::module_server::myserver::run_server;
use oat_lib::module_pubsub::pubsub::pubsub;

use std::thread;

pub fn main() {
    let mut processes = vec![];

    processes.push(thread::spawn(run_server));
    processes.push(thread::spawn(run_client));
    processes.push(thread::spawn(pubsub));

    for proc in processes {
        let _ = proc.join();
    }
}