use std::fmt::Debug;
use crate::writer::Writer;
use super::instruction::Instruction;

pub struct Description {
    pub constants_mask: u8,
    pub constants: u8,
    pub parse_fn: fn(&[u8]) -> Instruction,
    pub write_fn: fn(&mut Writer, &Instruction),
}

impl Description {
    pub fn matches(&self, byte: u8) -> bool {
        (byte & self.constants_mask) == self.constants
    }

    pub fn parse(&'static self, bytes: &[u8]) -> Instruction {
        (self.parse_fn)(bytes)
    }
}

impl Debug for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Description")
            .field("constants_mask", &self.constants_mask)
            .field("constants", &self.constants)
            .finish()
    }
}

pub mod descriptions {
    pub mod mov {
        use crate::instructions::mov::*;

        use super::super::Description;
        pub const TO_REGISTER: Description = Description {
            constants_mask: 0b11111100,
            constants: 0b10001000,
            parse_fn: parse_mov_to_register,
            write_fn: write_mov_to_register,
        };
        pub const IMMEDIATE_TO_MEMORY: Description = Description {
            constants_mask: 0b11111110,
            constants: 0b11000110,
            parse_fn: parse_mov_immediate_to_memory,
            write_fn: write_mov_immediate_to_memory,
        };
        pub const IMMEDIATE_TO_REGISTER: Description = Description {
            constants_mask: 0b11110000,
            constants: 0b10110000,
            parse_fn: parse_mov_immediate_to_register,
            write_fn: write_mov_immediate_to_register,
        };
    }
}

use descriptions::*;

pub const DESCRIPTIONS: &[&'static Description] = &[
    &mov::TO_REGISTER,
    &mov::IMMEDIATE_TO_MEMORY,
    &mov::IMMEDIATE_TO_REGISTER,
];
