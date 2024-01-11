pub mod room;
pub mod onlineswitcher;
use crate::home_router::device::room::socket::Socket;
use crate::home_router::device::room::termometr::Termometr;
#[derive(Debug, PartialEq)]
pub enum Device {
    SocketDevice(Socket),
    TermometrDevice(Termometr),
}
