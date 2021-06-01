use std::thread;

pub fn run_server() {
    let context = zmq::Context::new();
    let pubs = context.socket(zmq::PUB).unwrap();

    pubs.bind("ipc://stonk.pubsub.ipc").expect("failed binding publisher");

    println!("Starting server...");
    for _ in 1..15 {
        let zipcode = "pubsub";

        let update = format!("{} 'message to pubsub!'", zipcode);
        pubs.send(&update, 0).unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
    }
}