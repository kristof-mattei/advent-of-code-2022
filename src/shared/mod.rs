use std::{error, fmt};
#[derive(Debug)]
pub struct AoCError {
    pub message: String,
}

impl fmt::Display for AoCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl error::Error for AoCError {}

#[derive(Debug, PartialEq)]
pub enum AoCResult {
    Ofi32(i32),
    Ofu32(u32),
}

impl fmt::Display for AoCResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AoCResult::Ofi32(x) => write!(f, "{}", x),
            AoCResult::Ofu32(x) => write!(f, "{}", x),
        }
    }
}
