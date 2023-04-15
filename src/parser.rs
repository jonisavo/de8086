use crate::instructions::instruction::Instruction;
use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Parser<'a> {
    bytes: &'a [u8],
    current_index: usize,
}

#[derive(Debug)]
pub struct ParserInitError;

impl Error for ParserInitError {}

impl fmt::Display for ParserInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not initialize parser.")
    }
}

impl<'a> Parser<'a> {
    pub fn build(bytes: &'a [u8]) -> Result<Parser, ParserInitError> {
        if bytes.len() == 0 {
            return Err(ParserInitError);
        }

        Ok(Self {
            bytes,
            current_index: 0,
        })
    }
}

impl Iterator for Parser<'_> {
    type Item = Instruction;

    /// Parses the next instruction.
    fn next(&mut self) -> Option<Instruction> {
        let remaining_bytes = self.bytes.len().saturating_sub(self.current_index);

        if remaining_bytes == 0 {
            return None;
        }

        let remaining_bytes_slice = &self.bytes[self.current_index..];

        let parsed = Instruction::parse(remaining_bytes_slice)?;

        self.current_index += parsed.length as usize;

        Some(parsed)
    }
}
