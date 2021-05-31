pub fn run_server() {
    let context = zmq::Context::new();
    let pubs = context.socket(zmq::PUB).unwrap();

    pubs.bind("tcp://127.0.0.1:1234").expect("failed binding publisher");

    loop {
        let zipcode = 10001;

        let update = format!("{} asdf", zipcode);
        pubs.send(&update, 0).unwrap();
    }
}