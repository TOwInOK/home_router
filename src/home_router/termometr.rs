#[derive(PartialEq, Debug)]
pub struct Termometr {
    name: String,
    temperature: i8,
    online: bool,
}

impl Termometr {
    pub fn new(name: String, temperature: i8, online: bool) -> Self {
        Self { name, temperature, online}
    }
}
#[derive(PartialEq, Debug)]
pub struct TermometrtList {
    list: Vec<Termometr>,
}
impl TermometrtList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn add(&mut self, termometr: Termometr) {
        if !(self.list.contains(&termometr)) {
            self.list.push(termometr)
        } else {
            println!("This termometr is alredy created")
        }
    }
    //pull name and if it back we take it like turple
    pub fn get_info_by_name(&mut self, name: String) -> Option<(String, i8)> {
        for socket in &self.list {
            if socket.name == name {
                return Some(((socket.name.clone()), socket.temperature));
            }
        }
        None
    }
}

impl Default for TermometrtList {
    fn default() -> Self {
        Self::new()
    }
}
