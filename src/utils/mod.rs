//! Utilities provided by `Scion` to help to do some basic stuff.
pub mod file;
pub mod logger;
pub mod maths;

#[derive(Debug)]
pub struct ScionError {
    details: String
}

impl ScionError {
    pub fn new(msg: &str) -> Self {
        Self{details: msg.to_string()}
    }
}