pub mod home_router;
use home_router::{socket::*, termometr::*, *};
fn main() {
    let socket1 = Socket::new("SoketName".to_string(), 20, true);
    let termometr1 = Termometr::new("TermoName".to_string(), -20, true);
    let mut socket_list = SocketList::new();
    socket_list.add(socket1);
    let mut termometr_list = TermometrtList::new();
    termometr_list.add(termometr1);
    let room1 = Room::new("Room1".to_string(), socket_list, termometr_list);
    let mut library = Lib::new();
    library.add(room1);
    library.get_info();
}
