#[derive(PartialEq, Debug)]
pub struct Socket {
    uuid: String,
    power: u8,
    online: bool,
}
impl Socket {
    pub fn new(uuid: String, power: u8, online: bool) -> Self {
        Self { uuid, power, online}
    }
    pub fn online(&mut self) {
        self.online = !self.online
    }
}

