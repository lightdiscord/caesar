use std::io::{self, Read};
use std::env;
use crate::error::{Result, Error};

pub fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn read_shift() -> Result<u8> {
    let shift = env::args()
        .nth(1)
        .map(|shift| shift.parse::<u8>())
        .unwrap_or(Ok(0))?;

    if shift <= 25 {
        Ok(shift)
    } else {
        Err(Error::InvalidNumber)
    }
}