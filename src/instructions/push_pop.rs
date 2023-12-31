use crate::{writer::Writer, Instruction};

use super::{
    common::{
        create_single_byte_instruction, get_disp_value, get_displacement_amount, get_register,
        get_segment_register, InstructionDataFields,
    },
    opcode::Opcode,
    Description,
};

fn get_push_or_pop_opcode(byte: u8) -> Opcode {
    let middle_bytes = byte >> 3;
    if middle_bytes & 0b111 == 0b110 {
        Opcode::PUSH
    } else {
        Opcode::POP
    }
}

pub fn parse_push_pop_register_or_memory(bytes: &[u8], inst: &mut Instruction) {
    let displacement = get_displacement_amount(bytes[1]);

    inst.opcode = get_push_or_pop_opcode(bytes[1]);
    inst.length = 2 + displacement;
    inst.register = get_register(bytes[1] >> 3);
    inst.data_fields = InstructionDataFields::parse(bytes[1]);
    inst.disp = get_disp_value(bytes, displacement, 2);
}

pub fn write_push_or_pop(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_str("word ")
        .write_destination(instruction)
        .end_line();
}

pub const PUSH_POP_REGISTER_OR_MEMORY: Description = Description {
    write_fn: write_push_or_pop,
    parse_fn: parse_push_pop_register_or_memory,
};
pub const PUSH_REGISTER: Description = Description {
    write_fn: write_push_or_pop,
    parse_fn: |bytes, inst| {
        let register = get_register(bytes[0]);
        create_single_byte_instruction(inst, Opcode::PUSH, register)
    },
};
pub const POP_REGISTER: Description = Description {
    write_fn: write_push_or_pop,
    parse_fn: |bytes, inst| {
        let register = get_register(bytes[0]);
        create_single_byte_instruction(inst, Opcode::POP, register)
    },
};
pub const PUSH_SEGMENT_REGISTER: Description = Description {
    write_fn: write_push_or_pop,
    parse_fn: |bytes, inst| {
        let register = get_segment_register(bytes[0] >> 3);
        create_single_byte_instruction(inst, Opcode::PUSH, register)
    },
};
pub const POP_SEGMENT_REGISTER: Description = Description {
    write_fn: write_push_or_pop,
    parse_fn: |bytes, inst| {
        let register = get_segment_register(bytes[0] >> 3);
        create_single_byte_instruction(inst, Opcode::POP, register)
    },
};
