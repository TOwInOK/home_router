pub mod home_router;
use home_router::Comntroller;
use log::info;
fn main() {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Trace).init();
    let mut hub = Comntroller::new();
    hub.add("Test1".to_string());
    hub.add("Test2".to_string());
    hub.add("Test3".to_string());
    hub.add("Test4".to_string());
    hub.add_socket("Test1".to_string(), "name".to_string(), 254, false);
    hub.add_termometr("Test1".to_string(), "name2".to_string(), false, -20);
    hub.add_termometr("Test1".to_string(), "name3".to_string(), false, -20);
    hub.add_termometr("Test1".to_string(), "name5".to_string(), false, -20);
    hub.add_socket("Test1".to_string(), "nam32e".to_string(), 254, false);
    hub.add_socket("Test1".to_string(), "name323".to_string(), 254, false);
    //меняем строку состояния
    hub.online_switcher("name2".to_string(), "Test1".to_string());
    hub.add_socket("Test1".to_string(), "name32".to_string(), 254, false);
    hub.delete_room("Test2".to_string());
    info!("Получений raw info");
    hub.get_info();
    info!("Получаем отформатированный список комнат в хабе");
    hub.list();
    info!("Получаем список устройсв в конкретной комнате");
    hub.device_list("Test1".to_string());
    info!("Пример поиска устройства");
    hub.get_device_state("Test1".to_string(), "name".to_string());
    info!("Создаём отчёт по всем комнатам");
    let report = hub.create_report();
    info!("{}", report);
}
