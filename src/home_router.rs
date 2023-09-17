pub mod socket;
pub mod termometr;

use socket::*;
use termometr::*;

#[derive(PartialEq, Debug)]
pub struct Room {
    name: String,
    socketlist: SocketList,
    termometrlist: TermometrtList,
}
impl Room {
    pub fn new(name: String, socket_list: SocketList, termometr_list: TermometrtList) -> Self {
        Self {
            name,
            socketlist: socket_list,
            termometrlist: termometr_list,
        }
    }
}
#[derive(Debug)]
pub struct Lib {
    rooms: Vec<Room>,
}
impl Lib {
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }
    pub fn add(&mut self, room: Room) {
        if !(self.rooms.contains(&room)) {
            self.rooms.push(room)
        } else {
            println!("This room is alredy created")
        }
    }
    // сделайть выборку информации, полностью для одного обьекта
    pub fn get_info(&mut self) {
        for room in &self.rooms {
            println!("{:?}", &room);
        }
    }
    //need to find name in rooms and add in socket_list
    // pub fn new_soket() {
    //     !todo!()
    // }
}

impl Default for Lib {
    fn default() -> Self {
        Self::new()
    }
}
