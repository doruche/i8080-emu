#![allow(unused)]

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    UnknownOpcode(u8),
}

impl Display for Error {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            UnknownOpcode(opcode) => write!(f, "Invalid opcode {}.", opcode),
        }
    }
}

impl std::error::Error for Error {}