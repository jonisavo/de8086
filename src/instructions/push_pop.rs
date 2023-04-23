use crate::{writer::Writer, Instruction};

use super::{
    common::{
        create_single_byte_instruction, get_disp_value, get_displacement_amount, get_register,
        get_segment_register, InstructionDataFields, InstructionFields,
    },
    Description,
};

fn get_push_or_pop_mnemonic(byte: u8) -> &'static str {
    let middle_bytes = byte >> 3;
    if middle_bytes & 0b111 == 0b110 {
        "push"
    } else {
        "pop"
    }
}

pub fn parse_push_pop_register_or_memory(bytes: &[u8]) -> Instruction {
    let displacement = get_displacement_amount(bytes[1]);
    let disp = get_disp_value(&bytes, displacement, 2);

    Instruction {
        mnemonic: get_push_or_pop_mnemonic(bytes[1]),
        length: 2 + displacement,
        fields: InstructionFields::EMPTY,
        register: get_register(bytes[1] >> 3),
        data_fields: InstructionDataFields::parse(bytes[1]),
        disp,
        data: 0,
        description: &PUSH_POP_REGISTER_OR_MEMORY,
    }
}

pub fn write_push_or_pop(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_str("word ")
        .write_str(&instruction.destination_string())
        .end_line();
}

pub const PUSH_POP_REGISTER_OR_MEMORY: Description = Description {
    write_fn: write_push_or_pop,
    parse_fn: parse_push_pop_register_or_memory,
};
pub const PUSH_REGISTER: Description = Description {
    write_fn: write_push_or_pop,
    parse_fn: |bytes| {
        let register = get_register(bytes[0]);
        create_single_byte_instruction("push", &PUSH_REGISTER, register)
    },
};
pub const POP_REGISTER: Description = Description {
    write_fn: write_push_or_pop,
    parse_fn: |bytes| {
        let register = get_register(bytes[0]);
        create_single_byte_instruction("pop", &POP_REGISTER, register)
    },
};
pub const PUSH_SEGMENT_REGISTER: Description = Description {
    write_fn: write_push_or_pop,
    parse_fn: |bytes| {
        let register = get_segment_register(bytes[0] >> 3);
        create_single_byte_instruction("push", &PUSH_SEGMENT_REGISTER, register)
    },
};
pub const POP_SEGMENT_REGISTER: Description = Description {
    write_fn: write_push_or_pop,
    parse_fn: |bytes| {
        let register = get_segment_register(bytes[0] >> 3);
        create_single_byte_instruction("pop", &POP_SEGMENT_REGISTER, register)
    },
};
