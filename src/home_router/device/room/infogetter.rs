use std::fmt::Debug;
use crate::Comntroller;
use log::info as li;
///Get info about any object
pub trait GetInfo {
    fn get_info(&self);
}
impl<T: Debug> GetInfo for T {
   fn get_info(&self) {
        let output = format!("{:#?}", self);
        li!("{}", output);
    }
}

impl Default for Comntroller {
    fn default() -> Self {
        Self::new()
    }
}
