use std::thread;

pub fn pubsub() {
    let context = zmq::Context::new();
    let pubs = context.socket(zmq::PUB).unwrap();
    let sub = context.socket(zmq::SUB).unwrap();

    pubs.bind("ipc://stonk.sub.ipc").expect("failing binding pubsub publisher");
    sub.connect("ipc://stonk.pubsub.ipc").expect("failed connecting pubsub subscriber");

    let listen_string = "pubsub";
    println!("Starting pubsub...");
    assert!(sub.set_subscribe(listen_string.as_bytes()).is_ok());

    for _ in 1..15 {
        let topic_string = "sub";

        let string = sub.recv_string(0).unwrap().unwrap();
        println!("Received {} from pub", string);            

        let update = format!("{} 'sent from pubsub!'", topic_string);
        pubs.send(&update, 0).unwrap();
        thread::sleep(std::time::Duration::from_secs(1));

    }
}
