pub fn run_client() {
    println!("Collecting updates from server...");

    let context = zmq::Context::new();
    let sub = context.socket(zmq::SUB).unwrap();
    sub.connect("tcp://127.0.0.1:1234").expect("failed to connect to socket");

    let filter = "10001";
    assert!(sub.set_subscribe(filter.as_bytes()).is_ok());

    for _ in 1..100 {
        let string = sub.recv_string(0).unwrap().unwrap();
        println!("{}", string);
    }

}