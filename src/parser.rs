use std::error::Error;
use std::fmt;

use crate::instructions::description::Description;
use crate::instructions::instruction::Instruction;
use crate::DESCRIPTIONS;

pub struct Parser<'a> {
    sorted_descriptions: Vec<&'static Description>,
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

        let mut sorted_descriptions = DESCRIPTIONS.to_vec();

        sorted_descriptions.sort_by(|a, b| {
            let a_length = a.constants_length_bits;
            let b_length = b.constants_length_bits;

            if a_length > b_length {
                return std::cmp::Ordering::Less;
            }

            if a_length < b_length {
                return std::cmp::Ordering::Greater;
            }

            std::cmp::Ordering::Equal
        });

        Ok(Self {
            sorted_descriptions,
            bytes: bytes,
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

        let header = if remaining_bytes == 1 {
            self.bytes[self.current_index] as u16
        } else {
            (self.bytes[self.current_index] as u16) << 8 | self.bytes[self.current_index + 1] as u16
        };

        let remaining_bytes_slice = &self.bytes[self.current_index..];

        for description in &self.sorted_descriptions {
            if description.matches(header) {
                let parsed = description.parse(remaining_bytes_slice)?;

                self.current_index += parsed.length as usize;

                return Some(parsed);
            }
        }

        None
    }
}
