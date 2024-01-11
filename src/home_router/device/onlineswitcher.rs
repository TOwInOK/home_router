use std::fmt::{self, Display, Formatter};
use crate::home_router::device::Socket;
use crate::home_router::device::Termometr;
use crate::home_router::device::Device;
use log::info as li;
pub trait OnlineSwitcher {
    fn switch(&mut self);
}
impl OnlineSwitcher for Socket {
    fn switch(&mut self) {
        li!("Switch from {}", self.online);
        self.online = !self.online;
    }
}
impl OnlineSwitcher for Termometr {
    fn switch(&mut self) {
        li!("Switch from {}", self.online);
        self.online = !self.online;
    }
}
impl Display for Device {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Device::SocketDevice(socket) => {
                write!(
                    f,
                    "Soket: Power: {} | Online: {}",
                    socket.power, socket.online
                )
            }
            Device::TermometrDevice(termometr) => {
                write!(
                    f,
                    "Termomert: Temperature: {} | Online: {}",
                    termometr.temperature, termometr.online
                )
            }
        }
    }
}