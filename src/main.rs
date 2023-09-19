pub mod home_router;
use home_router::Comntroller;
fn main() {
    //убрать uuid
    let mut hub = Comntroller::new();
    hub.add("Test1".to_string());
    hub.add("Test2".to_string());
    hub.add("Test3".to_string());
    hub.add("Test4".to_string());
    // hub.get_info();
    hub.add_socket("Test1".to_string(), "name".to_string(), 254, false);
    hub.add_termometr("Test1".to_string(), "name2".to_string(), false, -20);
    // hub.online_switcher("name2".to_string(), "Test1".to_string());
    // hub.find_device("name2".to_string(), "Test1".to_string());
    // hub.add_socket("Test1".to_string(), "name32".to_string(), 254, false);
    // hub.delete_room("Test2".to_string());
    hub.get_info();
    hub.list();
    hub.device_list("Test1".to_string());
    hub.get_device_state("Test1".to_string(), "name".to_string());
}

trait DeviceStateProvider {
    fn get_device_state(&mut self, room_name: String, device_name: String);
}
impl DeviceStateProvider for Comntroller {
    fn get_device_state(&mut self, room_name: String, device_name: String) {
        match self.find_room(&room_name) {
            Ok(room) => match room.get_device_mut(&device_name) {
                Ok(device) => {
                    println!("Name: {} | {}", device_name, device)
                }
                Err(error) => println!("{error}"),
            },
            Err(error) => {
                println!("{error}");
            }
        }
    }
}
