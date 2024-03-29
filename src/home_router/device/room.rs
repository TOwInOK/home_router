pub mod socket;
pub mod termometr;
pub mod infogetter;
use log::info as li;
use socket::Socket;
use termometr::Termometr;
use std::collections::HashMap;
use thiserror::Error;
use crate::home_router::device::Device;
use crate::home_router::device::onlineswitcher::OnlineSwitcher;
#[derive(PartialEq, Debug)]
pub struct Room {
    pub name: String,
    devices: HashMap<String, Device>,
}
impl Room {
    //initialiseit new socket
    pub fn new(name: String, devices: HashMap<String, Device>) -> Self {
        li!("Create new room with name: {}", &name);
        li!("Devices in {} is: {:#?}", &name, devices);
        Self { name, devices }
    }
    //add socket override
    pub fn add_socket(&mut self, name: String, power: u8, online: bool) {
        li!("Get device list {:#?}", &Room::list(self));

        let socket = Socket::new(power, online);
        li!("Create socket: {:#?}", &socket);

        let device = Device::SocketDevice(socket);
        li!("Change device list to: {:#?}", &device);

        self.devices.insert(name, device);
    }
    //add termometr override
    pub fn add_termometr(&mut self, name: String, temperature: i8, online: bool) {
        let termometr = Termometr::new(temperature, online);
        let device = Device::TermometrDevice(termometr);
        self.devices.insert(name, device);
    }
    //remove object by Key(String)
    pub fn rm(&mut self, key: String) {
        self.devices.remove(&key);
    }
    //find some room by using name
    pub fn find_room_by_name(&self, name_to_find: &str) -> Option<&Room> {
        if self.name == name_to_find {
            Some(self)
        } else {
            None
        }
    }
    //Get mutable link about device
    pub fn get_device_mut(&mut self, key: &str) -> Result<&mut Device, Errors> {
        if let Some(device) = self.devices.get_mut(key) {
            Ok(device)
        } else {
            Err(Errors::NotFound)
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
    ///Get console output of the Devices into
    pub fn list(&self) -> String {
        let mut output: String = Default::default();
        for device in self.devices.iter().enumerate() {
            let d = device.1;
            let info = &format!("ID:{} | Name: {} | Type: {}\n", device.0, d.0, d.1);
            output += info;
        }
        output
    }
}

#[derive(Debug, Error)]
pub enum Errors {
    #[error("Nothing has found")]
    NotFound,
}
