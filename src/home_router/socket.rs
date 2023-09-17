#[derive(PartialEq, Debug)]
pub struct Socket {
    name: String,
    power: u8,
    online: bool,
}
//add default value
impl Socket {
    pub fn new(name: String, power: u8, online: bool) -> Self {
        Self { name, power, online}
    }
}
#[derive(PartialEq, Debug)]
pub struct SocketList {
    list: Vec<Socket>,
}

impl SocketList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn add(&mut self, socket: Socket) {
        if !(self.list.contains(&socket)) {
            self.list.push(socket)
        } else {
            println!("This soket is alredy created")
        }
    }
    pub fn get_info_by_name(&mut self, name: String) -> Option<(String, u8)> {
        for socket in &self.list {
            if socket.name == name {
                return Some(((socket.name.clone()), socket.power));
            }
        }
        None
    }
}
impl Default for SocketList {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for SocketList {
    type Item = Socket;
    type IntoIter = std::vec::IntoIter<Socket>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}