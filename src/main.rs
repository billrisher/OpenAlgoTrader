mod myserver;
mod client;

use std::thread;
use std::time::Duration;


use myserver::run_server;
use client::run_client;

fn main() {
    thread::spawn(run_server);
    run_client();
}
