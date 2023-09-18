#[derive(PartialEq, Debug)]
pub struct Termometr {
    uuid: String,
    temperature: i8,
    online: bool,
}

impl Termometr {
    pub fn new(uuid: String, temperature: i8, online: bool) -> Self {
        Self { uuid, temperature, online}
    }
    pub fn online(&mut self) {
        self.online = !self.online
    }
}