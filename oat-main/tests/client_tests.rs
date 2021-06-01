use oat;

#[test]
fn test_client_starts() {
    assert_eq!(2, oat::myserver::myserver::run_server());
}