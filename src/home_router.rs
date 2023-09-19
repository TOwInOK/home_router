pub mod room;
use std::{collections::HashMap, fmt::Debug};

use room::*;

#[derive(Debug)]
pub struct Comntroller {
    rooms: Vec<Room>,
}
impl Comntroller {
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }
    pub fn add(&mut self, name: String) {
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
    fn create_room(&mut self, name: String) -> Room {
        let hmap = HashMap::<String, Device>::new();
        let room: Room = Room::new(name, hmap);
        room
    }
    //Переделать в Ok|Err (если надо повышенная абстракция)
    pub fn delete_room(&mut self, room_name: String) {
        if let Some(index) = self
            .rooms
            .iter_mut()
            .position(|room| room.name == room_name)
        {
            self.rooms.remove(index);
            println!("Комната была удалена.")
        } else {
            println!("Данной комнаты не оказалось.")
        }
    }
    //если переделывать для клиент сервер, то отправлять Vec либо Hash, но не String
    //выводит все комнаты в хабе
    pub fn list(&self) {
        for local_room in self.rooms.iter().enumerate() {
            println!("ID: {} | Name: {}", &local_room.0, &local_room.1.name);
        }
    }
    //выводит все устройства в конкретной комнате
    pub fn device_list(&mut self, room_name: String) {
        match self.find_room(&room_name) {
            Ok(room) => room.list(),
            Err(error) => {
                println!("{error}")
            }
        }
    }
    //Добавляем розетку, поиск через функцию получения ссылки
    pub fn add_socket(&mut self, room_name: String, name: String, power: u8, online: bool) {
        match self.find_room(&room_name) {
            Ok(room) => match room.get_device_mut(&name) {
                Ok(_device) => {
                    println!("Устройство {} уже сушествует", &name)
                }
                Err(error) => {
                    println!("{}, было создано новое с именем {}", error, &name);
                    room.add_socket(name, power, online)
                }
            },
            Err(error) => {
                println!("{error}");
            }
        }
    }
    //термометр реализован по той же схеме что и добавление розетки
    pub fn add_termometr(
        &mut self,
        room_name: String,
        name: String,
        online: bool,
        temperature: i8,
    ) {
        match self.find_room(&room_name) {
            Ok(room) => match room.get_device_mut(&name) {
                Ok(_device) => {
                    println!("Устройство {} уже сушествует", &name)
                }
                Err(error) => {
                    println!("{}, было создано новое с именем {}", error, &name);
                    room.add_termometr(name, temperature, online)
                }
            },
            Err(error) => {
                println!("{error}");
            }
        }
    }
    //Функция для поиска комнаты
    pub fn find_room(&mut self, name: &str) -> Result<&mut Room, &'static str> {
        if let Some(room) = self
            .rooms
            .iter_mut()
            .find(|room| room.find_room_by_name(name).is_some())
        {
            Ok(room)
        } else {
            Err("Комната не была найдена")
        }
    }
    //Функция для поиска устройства по ключу и вывода на экран
    // Аналог трейта
    pub fn get_device_state(&mut self, room_name: String, device_name: String) {
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
    // Меняем тумблер включения устройства
    pub fn online_switcher(&mut self, key: String, room_name: String) {
        match self.find_room(&room_name) {
            Ok(room) => room.online_switcher(&key),
            Err(error) => {
                println!("{error}");
            }
        }
    }
}
//выводим отладочную инфу о всех типах
//используем для вывода как "json" для логов.
trait GetInfo {
    fn get_info(&self);
}
impl<T: Debug> GetInfo for T {
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

impl Default for Comntroller {
    fn default() -> Self {
        Self::new()
    }
}
