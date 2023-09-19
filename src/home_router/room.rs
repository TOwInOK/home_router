pub mod socket;
pub mod termometr;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use socket::*;
use termometr::*;
#[derive(Debug, PartialEq)]
pub enum Device {
    SocketDevice(Socket),
    TermometrDevice(Termometr),
}

#[derive(PartialEq, Debug)]
pub struct Room {
    pub name: String,
    devices: HashMap<String, Device>,
}
impl Room {
    pub fn new(name: String, devices: HashMap<String, Device>) -> Self {
        Self { name, devices }
    }
    //add socket override
    pub fn add_socket(&mut self, name: String, power: u8, online: bool) {
        let socket = Socket::new(power, online);
        let device = Device::SocketDevice(socket);
        self.devices.insert(name, device);
    }
    //add termometr override
    pub fn add_termometr(&mut self, name: String, temperature: i8, online: bool) {
        let termometr = Termometr::new(temperature, online);
        let device = Device::TermometrDevice(termometr);
        self.devices.insert(name, device);
    }
    //remove object by Key(String)
    pub fn rm(&mut self, name: String) {
        self.devices.remove(&name);
    }
    pub fn find_room_by_name(&self, name_to_find: &str) -> Option<&Room> {
        if self.name == name_to_find {
            Some(self)
        } else {
            None
        }
    }
    //Выдаём мутабельную ссылку на устройство
    pub fn get_device_mut(&mut self, key: &str) -> Result<&mut Device, &'static str> {
        if let Some(device) = self.devices.get_mut(key) {
            Ok(device)
        } else {
            Err("Устройство не найдено")
        }
    }
    //Меняем орие.. состояние устройства
    pub fn online_switcher(&mut self, key: &str) {
        match self
            .get_device_mut(key)
            .expect("Смена включения оборволась так как устройство оказалось не верным")
        {
            Device::SocketDevice(socket) => socket.switch(),
            Device::TermometrDevice(termometr) => termometr.switch(),
        }
    }
    pub fn list(&self) {
        for device in self.devices.iter().enumerate() {
            let d = device.1;
            println!("ID:{} | Name: {} | Type: {}", device.0, d.0, d.1);
        }
    }
}

trait OnlineSwitcher {
    fn switch(&mut self);
}
impl OnlineSwitcher for Socket {
    fn switch(&mut self) {
        self.online = !self.online;
    }
}
impl OnlineSwitcher for Termometr {
    fn switch(&mut self) {
        self.online = !self.online;
    }
}
impl Display for Device {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Device::SocketDevice(socket) => {
                write!(f, "Power: {} | Online: {}", socket.power, socket.online)
            }
            Device::TermometrDevice(termometr) => {
                write!(
                    f,
                    "Temperature: {} | Online: {}",
                    termometr.temperature, termometr.online
                )
            }
        }
    }
}
