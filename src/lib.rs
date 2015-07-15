// Externs
extern crate sysconf;


// Imports
use std::io;
use std::result;
use std::num::ParseIntError;


// Exports
pub mod cpu;
pub mod process;


pub type Result<T> = result::Result<T, ProcureError>;


#[derive(Debug)]
pub enum ProcureError {
    RuntimeError(String),
    IoError(io::Error),
    ParseError(ParseIntError),
}

