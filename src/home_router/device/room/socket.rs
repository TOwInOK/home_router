#[derive(PartialEq, Debug)]
pub struct Socket {
    pub power: u8,
    pub online: bool,
}
impl Socket {
    pub fn new(power: u8, online: bool) -> Self {
        Self { power, online }
    }
}
