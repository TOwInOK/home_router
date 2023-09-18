pub mod room;
use std::{fmt::Debug, collections::HashMap};

use room::*;

#[derive(Debug)]
pub struct Comntroller {
    rooms: Vec<Room>,
}
impl Comntroller {
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }
    pub fn add(&mut self, name:String) {
        let room = self.create_room(name);
        if !(self.rooms.contains(&room)) {
            self.rooms.push(room)
        } else {
            println!("Комната уже создана")
        }
    }
    //Выборка информации для всего
    pub fn get_info(&mut self) {
        for room in &self.rooms {
            room.get_info()
        }
    }
    //Добавление комнаты
    fn create_room(&mut self, name:String) -> Room{
        let hmap = HashMap::<String, Device>::new();
        let room: Room = Room::new(name, hmap);
        room
    }
    pub fn add_socket (&mut self,room_name: String, name: String, uuid: String, power: u8, online: bool) {
        let room = self.find_room(&room_name);
        if let Some(_device) = room.get_device_mut(&name) {
            println!("Устройство {} уже сушествует", &name);
            return;
        }
        room.add_socket(name, uuid, power, online)
    }
    pub fn add_termometr (&mut self,room_name: String, name: String, uuid: String, online: bool, temperature: i8) {
        let room = self.find_room(&room_name);
        if let Some(_device) = room.get_device_mut(&name) {
            println!("Устройство {} уже сушествует", &name);
            return;
        }
        room.add_termometr(name, uuid, temperature, online)
    }
    //Функция для поиска комнаты
    // Переписать под &mut Room
    fn find_room(&mut self, name: &str) -> &mut Room {
        self.rooms.iter_mut().find(|room| room.find_room_by_name(&name).is_some()).expect("Комната не была найдена")
    }
    //Функция для поиска устройства по ключу и вывода на экран
    pub fn find_device (&mut self, key: String, room_name:String) {
        let room = self.find_room(&room_name);
        room.get_device_mut(&key).expect("Устройство не было обнаружено").get_info()
    }
    pub fn online_switcher(&mut self, key: String, room_name:String) {
        let room = self.find_room(&room_name);
        room.online_switcher(&key) 
    }
}
trait GetInfo {
    fn get_info(&self);
}
impl <T: Debug> GetInfo for T {
    fn get_info(&self) {
        let output = format!("||| {:?} |||", self);
        for _ in 0..output.len() {
            print!("-")
        }
        println!();
        println!("{}", output);
        for _ in 0..output.len() {
            print!("-")
        }
        println!();
    }
}