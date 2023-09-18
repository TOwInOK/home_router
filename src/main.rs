pub mod home_router;
use home_router::Comntroller;
fn main() {
    //убрать uuid
    let mut hub = Comntroller::new();
    hub.add("Test1".to_string());
    hub.get_info();
    hub.add_socket("Test1".to_string(), "name".to_string(), "4132f".to_string(), 254, false);
    hub.add_termometr("Test1".to_string(), "name2".to_string(), "r32fecr".to_string(), false, -20);
    hub.online_switcher("name2".to_string(), "Test1".to_string());
    hub.find_device("name2".to_string(), "Test1".to_string());
    hub.add_socket("Test1".to_string(), "name32".to_string(), "4132f".to_string(), 254, false);
    hub.get_info();
}
