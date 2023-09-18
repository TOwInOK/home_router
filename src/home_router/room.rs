pub mod socket;
pub mod termometr;
use std::collections::HashMap;

use socket::*;
use termometr::*;
#[derive(Debug, PartialEq)]
pub enum Device {
    SocketDevice(Socket),
    TermometrDevice(Termometr),
}

#[derive(PartialEq, Debug)]
pub struct Room {
    name: String,
    devices: HashMap<String, Device>
}
impl Room {
    pub fn new(name: String, devices:HashMap<String, Device>) -> Self {
        Self {
            name,
            devices,
        }
    }
    //add socket override
    pub fn add_socket(&mut self, name: String, uuid: String, power: u8, online: bool){
        let socket = Socket::new(uuid, power, online);
        let device = Device::SocketDevice(socket);
        self.devices.insert(name, device);
    }
    //add termometr override
    pub fn add_termometr(&mut self, name: String, uuid: String, temperature: i8, online: bool){
        let termometr = Termometr::new(uuid, temperature, online);
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
    pub fn get_device_mut(&mut self, key: &str) -> Option<&mut Device> {
        self.devices.get_mut(key)
    }
    pub fn online_switcher(&mut self, key: &str) {
        match self.get_device_mut(key).expect("Смена включения оборволась так как устройство оказалось не верным") {
            Device::SocketDevice(socket) => {
                socket.online()
            }
            Device::TermometrDevice(termometr) => {
                termometr.online()
            }
        }
    }
}

