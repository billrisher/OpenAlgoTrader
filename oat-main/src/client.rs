pub mod client {
    pub fn run_client() {

        println!("Collecting updates from server...");

        let context = zmq::Context::new();
        let sub = context.socket(zmq::SUB).unwrap();
        sub.connect("ipc://stonk.sub.ipc").expect("failed to connect to socket");

        let filter = "sub";
        assert!(sub.set_subscribe(filter.as_bytes()).is_ok());

        for _ in 1..15 {
            let string = sub.recv_string(0).unwrap().unwrap();
            println!("{}", string);
        }

    }
}