use super::{
    arithmetic::{self, add, cmp, sub},
    data_transfer,
    instruction::Instruction,
    jumps, mov, push_pop,
};
use crate::writer::Writer;
use std::fmt::Debug;

pub struct Description {
    pub parse_fn: fn(&[u8], &mut Instruction),
    pub write_fn: fn(&mut Writer, &Instruction),
}

impl Description {
    pub fn parse(&'static self, bytes: &[u8], inst: &mut Instruction) {
        (self.parse_fn)(bytes, inst)
    }
}

impl Debug for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Description").finish_non_exhaustive()
    }
}

fn unimplemented_parse(_bytes: &[u8], inst: &mut Instruction) {
    inst.mnemonic = "unimplemented";
    inst.length = 0;
    inst.description = &UNIMPLEMENTED;
}

fn unimplemented_write(_writer: &mut Writer, _instruction: &Instruction) {
    unimplemented!()
}

pub const UNIMPLEMENTED: Description = Description {
    parse_fn: unimplemented_parse,
    write_fn: unimplemented_write,
};

pub fn resolve(bytes: &[u8]) -> &'static Description {
    match bytes[0] {
        0b10001000..=0b10001011 => &mov::TO_REGISTER,
        0b11000110 | 0b11000111 => &mov::IMMEDIATE_TO_MEMORY,
        0b10110000..=0b10111111 => &mov::IMMEDIATE_TO_REGISTER,
        0b10100000..=0b10100011 => &mov::MEMORY_TO_ACCUMULATOR,
        0b10001100 | 0b10001110 => &mov::TO_SEGMENT_REGISTER,
        0b11111111 => &push_pop::PUSH_POP_REGISTER_OR_MEMORY,
        0b01010000..=0b01010111 => &push_pop::PUSH_REGISTER,
        0b00000110 | 0b00001110 | 0b00010110 | 0b00011110 => &push_pop::PUSH_SEGMENT_REGISTER,
        0b10001111 => &push_pop::PUSH_POP_REGISTER_OR_MEMORY,
        0b01011000..=0b01011111 => &push_pop::POP_REGISTER,
        0b00000111 | 0b00001111 | 0b00010111 | 0b00011111 => &push_pop::POP_SEGMENT_REGISTER,
        0b10000110 | 0b10000111 => &data_transfer::XCHG_MEMORY_WITH_REGISTER,
        0b10010000..=0b10010111 => &data_transfer::XCHG_REGISTER_WITH_ACCUMULATOR,
        0b11100100..=0b11100111 => &data_transfer::IN_OUT_FIXED_PORT,
        0b11101100..=0b11101111 => &data_transfer::IN_OUT_VARIABLE_PORT,
        0b11010111 | 0b10011111 | 0b10011110 | 0b10011100 | 0b10011101 => {
            &data_transfer::OTHER_DATA_TRANSFER
        }
        0b10001101 | 0b11000101 | 0b11000100 => &data_transfer::LEA_LDS_LES,
        0b00000000..=0b00000011 => &add::TO_REGISTER,
        0b10000000..=0b10000011 => &arithmetic::IMMEDIATE_TO_REGISTER_MEMORY,
        0b00000100 | 0b00000101 => &add::IMMEDIATE_TO_ACCUMULATOR,
        0b00101000..=0b00101011 => &sub::TO_REGISTER,
        0b00101100 | 0b00101101 => &sub::IMMEDIATE_FROM_ACCUMULATOR,
        0b00111000..=0b00111011 => &cmp::TO_REGISTER,
        0b00111100 | 0b00111101 => &cmp::IMMEDIATE_WITH_ACCUMULATOR,
        0b01110100 | 0b01111100 | 0b01111110 | 0b01110010 | 0b01110110 | 0b01111010
        | 0b01110000 | 0b01111000 | 0b01110101 | 0b01111101 | 0b01111111 | 0b01110011
        | 0b01110111 | 0b01111011 | 0b01110001 | 0b01111001 | 0b11100010 | 0b11100001
        | 0b11100000 | 0b11100011 => &jumps::CONDITIONAL_JUMP,
        _ => &UNIMPLEMENTED,
    }
}
