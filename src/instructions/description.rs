use std::io::{BufWriter, Stdout};

use super::instruction::Instruction;

pub struct Description {
    pub constants_length_bits: u8,
    pub constants_mask: u16,
    pub constants: u16,
    pub parse_fn: fn(&[u8]) -> Option<Instruction>,
    pub write_fn: fn(&mut BufWriter<Stdout>, &Instruction),
}

impl Description {
    pub fn matches(&self, instruction: u16) -> bool {
        (instruction & self.constants_mask) == self.constants
    }

    pub fn parse(&'static self, bytes: &[u8]) -> Option<Instruction> {
        (self.parse_fn)(bytes)
    }
}

pub mod descriptions {
    pub mod mov {
        use crate::instructions::mov::{parse_mov_to_register, write_mov_to_register};

        use super::super::Description;
        pub const TO_REGISTER: Description = Description {
            constants_length_bits: 6,
            constants_mask: 0b1111110000000000,
            constants: 0b1000100000000000,
            parse_fn: parse_mov_to_register,
            write_fn: write_mov_to_register,
        };
    }
}

pub const DESCRIPTIONS: &[&Description] = &[&descriptions::mov::TO_REGISTER];
