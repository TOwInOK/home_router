pub mod device;
use log::{error as le, info as li};
use std::{collections::HashMap, fmt::Debug};
use crate::home_router::device::room::Room;
use crate::home_router::device::room::infogetter::GetInfo;
use crate::home_router::device::Device;
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
            li!("Комната была создана: {:#?}", &room);
            self.rooms.push(room)
        } else {
            le!("Комната уже создана")
        }
    }
    ///Выборка информации для всего
    pub fn get_info(&mut self) {
        for room in &self.rooms {
            room.get_info()
        }
    }
    ///Добавление комнаты
    fn create_room(&mut self, name: String) -> Room {
        let hmap = HashMap::<String, Device>::new();
        let room: Room = Room::new(name, hmap);
        room
    }
    ///Удаляем комнату
    pub fn delete_room(&mut self, room_name: String) {
        if let Some(index) = self
            .rooms
            .iter_mut()
            .position(|room| room.name == room_name)
        {
            self.rooms.remove(index);
            li!("Комната была удалена.")
        } else {
            le!("Данной комнаты не оказалось.")
        }
    }
    ///Выводит все комнаты в хабе
    pub fn list(&self) {
        for local_room in self.rooms.iter().enumerate() {
            li!("ID: {} | Name: {}", &local_room.0, &local_room.1.name);
        }
    }
    ///Выводит все устройства в конкретной комнате
    pub fn device_list(&mut self, room_name: String) {
        match self.find_room(&room_name) {
            Ok(room) => li!("Room name: {}\nDevices:\n{}", room_name, room.list()),
            Err(error) => {
                le!("{error}")
            }
        }
    }
    ///Добавляем розетку, поиск через функцию получения ссылки
    pub fn add_socket(&mut self, room_name: String, name: String, power: u8, online: bool) {
        match self.find_room(&room_name) {
            Ok(room) => match room.get_device_mut(&name) {
                Ok(_device) => {
                    li!("Устройство {} уже сушествует", &name)
                }
                Err(error) => {
                    le!("{}, было создано новое с именем {}", error, &name);
                    room.add_socket(name, power, online)
                }
            },
            Err(error) => {
                le!("{error}");
            }
        }
    }
    ///Термометр реализован по той же схеме что и добавление розетки
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
                    li!("Устройство {} уже сушествует", &name)
                }
                Err(error) => {
                    le!("{}, было создано новое с именем {}", error, &name);
                    room.add_termometr(name, temperature, online)
                }
            },
            Err(error) => {
                le!("{error}");
            }
        }
    }
    ///Функция для поиска комнаты
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
    ///Функция для поиска устройства по ключу и вывода на экран
    pub fn get_device_state(&mut self, room_name: String, device_name: String) {
        match self.find_room(&room_name) {
            Ok(room) => match room.get_device_mut(&device_name) {
                Ok(device) => {
                    li!("Name: {} | {}", device_name, device)
                }
                Err(e) => le!("{e}"),
            },
            Err(e) => {
                le!("{e}");
            }
        }
    }
    /// Меняем тумблер включения устройства
    pub fn online_switcher(&mut self, key: String, room_name: String) {
        match self.find_room(&room_name) {
            Ok(room) => room.online_switcher(&key),
            Err(error) => {
                le!("{error}");
            }
        }
    }
    ///Создаёться отчёт по структуре
    pub fn create_report(&mut self) -> String {
        let mut output: String = Default::default();
        output += "START\n\n";
        for local_room in self.rooms.iter_mut() {
            output += &format!(
                "Room name: {}\nDevices:\n{}\n",
                local_room.name,
                local_room.list()
            )
        }
        output += "END\n";
        output
    }
}


