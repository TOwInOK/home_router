#[derive(PartialEq, Debug)]
pub struct Termometr {
    pub temperature: i8,
    pub online: bool,
}
impl Termometr {
    pub fn new(temperature: i8, online: bool) -> Self {
        Self {
            temperature,
            online,
        }
    }
}
