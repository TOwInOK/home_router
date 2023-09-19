pub mod home_router;
use home_router::Comntroller;
fn main() {
    let mut hub = Comntroller::new();
    hub.add("Test1".to_string());
    hub.add("Test2".to_string());
    hub.add("Test3".to_string());
    hub.add("Test4".to_string());
    hub.add_socket("Test1".to_string(), "name".to_string(), 254, false);
    hub.add_termometr("Test1".to_string(), "name2".to_string(), false, -20);
    //меняем строку состояния
    hub.online_switcher("name2".to_string(), "Test1".to_string());
    hub.add_socket("Test1".to_string(), "name32".to_string(), 254, false);
    hub.delete_room("Test2".to_string());
    println!("Получений raw info");
    hub.get_info();
    println!("Получаем отформатированный список комнат в хабе");
    hub.list();
    println!("Получаем список устройсв в конкретной комнате");
    hub.device_list("Test1".to_string());
    println!("Пример поиска устройства");
    hub.get_device_state("Test1".to_string(), "name".to_string());
}

// trait DeviceStateProvider {
//     fn get_device_state(&mut self, room_name: String, device_name: String);
// }
// //аналог функции...
// impl DeviceStateProvider for Comntroller {
//     fn get_device_state(&mut self, room_name: String, device_name: String) {
//         match self.find_room(&room_name) {
//             Ok(room) => match room.get_device_mut(&device_name) {
//                 Ok(device) => {
//                     println!("Name: {} | {}", device_name, device)
//                 }
//                 Err(error) => println!("{error}"),
//             },
//             Err(error) => {
//                 println!("{error}");
//             }
//         }
//     }
// }
