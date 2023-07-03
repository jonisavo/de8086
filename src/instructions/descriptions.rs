use super::{
    arithmetic, control_transfer, data_transfer, instruction::Instruction, logic, mov,
    opcode::Opcode, push_pop, strings,
};
use crate::{instructions::processor_control, writer::Writer};
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

pub const UNIMPLEMENTED: Description = Description {
    parse_fn: |_bytes, inst| {
        inst.opcode = Opcode::UNKNOWN;
        inst.length = 0;
    },
    write_fn: |_writer, instruction| unimplemented!("{:?}", instruction.opcode),
};

fn resolve_logic_bytes(bytes: &[u8]) -> &'static Description {
    let opcode = bytes[1] >> 3;

    match opcode & 0b111 {
        0b000 => &logic::ROL,
        0b001 => &logic::ROR,
        0b010 => &logic::RCL,
        0b011 => &logic::RCR,
        0b100 => &logic::SAL,
        0b101 => &logic::SHR,
        0b111 => &logic::SAR,
        _ => &UNIMPLEMENTED,
    }
}

fn resolve_f6_or_f7_byte(bytes: &[u8]) -> &'static Description {
    let opcode = bytes[1] >> 3;

    match opcode & 0b111 {
        0b000 => &logic::TEST_IMMEDIATE_AND_REGISTER_OR_MEMORY,
        0b010 => &logic::NOT,
        0b011 => &arithmetic::NEG,
        0b100 => &arithmetic::MUL,
        0b101 => &arithmetic::IMUL,
        0b110 => &arithmetic::DIV,
        0b111 => &arithmetic::IDIV,
        _ => &UNIMPLEMENTED,
    }
}

fn resolve_fe_byte(bytes: &[u8]) -> &'static Description {
    let opcode = bytes[1] >> 3;

    match opcode & 0b111 {
        0b000 => &arithmetic::INC_REGISTER_OR_MEMORY,
        0b001 => &arithmetic::DEC_REGISTER_OR_MEMORY,
        _ => &UNIMPLEMENTED,
    }
}

fn resolve_ff_byte(bytes: &[u8]) -> &'static Description {
    let opcode = bytes[1] >> 3;

    match opcode & 0b111 {
        0b000 => &arithmetic::INC_REGISTER_OR_MEMORY,
        0b001 => &arithmetic::DEC_REGISTER_OR_MEMORY,
        0b010 | 0b100 => &control_transfer::INDIRECT_WITHIN_SEGMENT,
        0b011 | 0b101 => &control_transfer::INDIRECT_INTERSEGMENT,
        0b110 => &push_pop::PUSH_POP_REGISTER_OR_MEMORY,
        _ => &UNIMPLEMENTED,
    }
}

pub fn resolve(bytes: &[u8]) -> &'static Description {
    assert!(bytes.len() > 0, "Cannot resolve instruction with no bytes.");

    match bytes[0] {
        0b10001000..=0b10001011 => &mov::TO_REGISTER,
        0b11000110 | 0b11000111 => &mov::IMMEDIATE_TO_MEMORY,
        0b10110000..=0b10111111 => &mov::IMMEDIATE_TO_REGISTER,
        0b10100000..=0b10100011 => &mov::MEMORY_TO_ACCUMULATOR,
        0b10001100 | 0b10001110 => &mov::TO_SEGMENT_REGISTER,
        0b11010000..=0b11010011 => resolve_logic_bytes(bytes),
        0b11110110 | 0b11110111 => resolve_f6_or_f7_byte(bytes),
        0b11111110 => resolve_fe_byte(bytes),
        0b11111111 => resolve_ff_byte(bytes),
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
        0b00000000..=0b00000011 => &arithmetic::ADD_TO_REGISTER,
        0b10000000..=0b10000011 => &arithmetic::IMMEDIATE_TO_REGISTER_MEMORY,
        0b00000100 | 0b00000101 => &arithmetic::ADD_IMMEDIATE_TO_ACCUMULATOR,
        0b00010000..=0b00010011 => &arithmetic::ADC_TO_REGISTER,
        0b00010100 | 0b00010101 => &arithmetic::ADC_IMMEDIATE_TO_ACCUMULATOR,
        0b01000000..=0b01000111 => &arithmetic::INC_REGISTER,
        0b00110111 => &arithmetic::AAA,
        0b00100111 => &arithmetic::DAA,
        0b00101000..=0b00101011 => &arithmetic::SUB_FROM_REGISTER,
        0b00101100 | 0b00101101 => &arithmetic::SUB_IMMEDIATE_FROM_ACCUMULATOR,
        0b00011000..=0b00011011 => &arithmetic::SBB_FROM_REGISTER,
        0b00011100 | 0b00011101 => &arithmetic::SBB_IMMEDIATE_FROM_ACCUMULATOR,
        0b01001000..=0b01001111 => &arithmetic::DEC_REGISTER,
        0b00111000..=0b00111011 => &arithmetic::CMP_WITH_REGISTER,
        0b00111100 | 0b00111101 => &arithmetic::CMP_IMMEDIATE_WITH_ACCUMULATOR,
        0b00111111 => &arithmetic::AAS,
        0b00101111 => &arithmetic::DAS,
        0b11010100 => &arithmetic::AAM,
        0b11010101 => &arithmetic::AAD,
        0b10011000 => &arithmetic::CBW,
        0b10011001 => &arithmetic::CWD,
        0b00100000..=0b00100011 => &logic::AND_WITH_REGISTER,
        0b00100100 | 0b00100101 => &logic::AND_IMMEDIATE_FROM_ACCUMULATOR,
        0b10000100..=0b10000111 => &logic::TEST_REGISTER_OR_MEMORY,
        0b10101000 | 0b10101001 => &logic::TEST_IMMEDIATE_AND_ACCUMULATOR,
        0b00001000..=0b00001011 => &logic::OR_WITH_REGISTER,
        0b00001100 | 0b00001101 => &logic::OR_IMMEDIATE_TO_ACCUMULATOR,
        0b00110000..=0b00110011 => &logic::XOR_WITH_REGISTER,
        0b00110100 | 0b00110101 => &logic::XOR_IMMEDIATE_TO_ACCUMULATOR,
        0b11110010 | 0b11110011 => &strings::REPEAT,
        0b10100100 | 0b10100101 => &strings::STRING_MANIPULATION, // movsb, movsw
        0b10100110 | 0b10100111 => &strings::STRING_MANIPULATION, // cmpsb, cmpsw
        0b10101110 | 0b10101111 => &strings::STRING_MANIPULATION, // scasb, scasw
        0b10101100 | 0b10101101 => &strings::STRING_MANIPULATION, // lodsb, lodsw
        0b10101010 | 0b10101011 => &strings::STRING_MANIPULATION, // stosb, stosw
        0b11101000 | 0b11101001 => &control_transfer::DIRECT_WITHIN_SEGMENT,
        0b11101011 => &control_transfer::JUMP_DIRECT_WITHIN_SEGMENT_SHORT,
        0b10011010 | 0b11101010 => &control_transfer::DIRECT_INTERSEGMENT,
        0b11000011 | 0b11001011 => &control_transfer::RETURN_NO_VALUE,
        0b11000010 | 0b11001010 => &control_transfer::RETURN_WITH_VALUE,
        0b01110100 | 0b01111100 | 0b01111110 | 0b01110010 | 0b01110110 | 0b01111010
        | 0b01110000 | 0b01111000 | 0b01110101 | 0b01111101 | 0b01111111 | 0b01110011
        | 0b01110111 | 0b01111011 | 0b01110001 | 0b01111001 | 0b11100010 | 0b11100001
        | 0b11100000 | 0b11100011 => &control_transfer::CONDITIONAL_JUMP,
        0b11001100..=0b11001111 => &control_transfer::INTERRUPT,
        0b11111000 | 0b11110101 | 0b11111001 | 0b11111100 | 0b11111101 | 0b11111010
        | 0b11111011 | 0b11110100 | 0b10011011 => &processor_control::PROCESSOR_CONTROL,
        0b11110000 => &processor_control::LOCK,
        0b00100110 | 0b00101110 | 0b00110110 | 0b00111110 => &processor_control::SEGMENT_OVERRIDE,
        _ => &UNIMPLEMENTED,
    }
}
