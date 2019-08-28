
use crate::result::{Result, Error};
use std::ffi::OsStr;

#[derive(Debug)]
pub struct Plugin {
}

impl Plugin {
    pub fn load<P: AsRef<OsStr>>(_path: P) -> Result<Plugin> {
        Err(Error::from("TODO"))
    }

    pub fn name(&self) -> &'static str {
        "TODO"
    }
}
