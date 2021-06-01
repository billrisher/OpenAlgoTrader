use oat_lib;

#[test]
fn test_client_starts() {
    assert_eq!(2, oat_lib::module_server::myserver::run_server());
}