mod error;
mod reader;

use error::Result;

fn shift_char(src: char, shift: u8) -> char {
    if !src.is_ascii_alphabetic() {
        return src;
    }

    let lowercase = src.is_ascii_lowercase();
    let case_offset = if lowercase { 'a' } else { 'A' } as u8;
    let value = (src as u8 - case_offset + shift) % 26;

    char::from(value + case_offset)
}

fn main() -> Result<()> {
    let buffer = reader::read_stdin()?;
    let shift = reader::read_shift()?;

    let result: String = buffer.chars()
        .map(|c| shift_char(c, shift))
        .collect();

    print!("{}", result);

    Ok(())
}
