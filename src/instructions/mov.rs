use crate::writer::Writer;

use super::{
    common::{
        get_data_value, get_disp_value, get_displacement_amount, get_register,
        get_segment_register,
        instruction_flags::{self, has_word_flag},
        parse_instruction_flags, parse_typical_instruction, register, write_immediate_instruction,
        write_typical_instruction, InstRegister, InstructionDataFields,
    },
    instruction::Instruction,
    opcode::Opcode,
    Description,
};

pub fn write_mov_immediate_to_memory(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_rm(instruction)
        .write_comma_separator()
        .write_with_size(instruction.data, instruction)
        .end_line();
}

pub fn parse_mov_immediate_to_memory(bytes: &[u8], inst: &mut Instruction) {
    let flags = parse_instruction_flags(bytes[0]);
    let displacement = get_displacement_amount(bytes[1]);
    let has_word_flag = has_word_flag(flags);
    let immediate_length = has_word_flag as u8 + 1;
    let data = get_data_value(bytes, has_word_flag, 2 + displacement as usize);

    inst.opcode = Opcode::MOV;
    inst.length = 2 + displacement + immediate_length;
    inst.flags = flags;
    inst.register = get_register(bytes[1] >> 3);
    inst.data_fields = InstructionDataFields::parse(bytes[1]);
    inst.disp = get_disp_value(bytes, displacement, 2);
    inst.data = data;
}

pub fn parse_mov_immediate_to_register(bytes: &[u8], inst: &mut Instruction) {
    let flags: u8 = parse_instruction_flags(bytes[0] >> 3);
    let has_word_flag = has_word_flag(flags);
    let length = has_word_flag as u8 + 2;
    let data = get_data_value(bytes, has_word_flag, 1);

    inst.opcode = Opcode::MOV;
    inst.length = length;
    inst.flags = flags;
    inst.register = get_register(bytes[0]);
    inst.data = data;
}

pub fn parse_mov_memory_to_accumulator(bytes: &[u8], inst: &mut Instruction) {
    let mut flags = parse_instruction_flags(bytes[0]);
    flags ^= instruction_flags::DIRECTION;

    inst.opcode = Opcode::MOV;
    inst.length = 3;
    inst.flags = flags;
    inst.register = InstRegister::Reg(register::AX);
    inst.data_fields = InstructionDataFields::DIRECT_ADDRESS;
    inst.disp = get_disp_value(bytes, 2, 1);
}

pub fn parse_mov_to_segment_register(bytes: &[u8], inst: &mut Instruction) {
    parse_typical_instruction(inst, Opcode::MOV, bytes);

    inst.register = get_segment_register(bytes[1] >> 3);
    inst.flags |= instruction_flags::WORD;
}

pub const TO_REGISTER: Description = Description {
    parse_fn: |b, inst| parse_typical_instruction(inst, Opcode::MOV, b),
    write_fn: write_typical_instruction,
};
pub const IMMEDIATE_TO_MEMORY: Description = Description {
    parse_fn: parse_mov_immediate_to_memory,
    write_fn: write_mov_immediate_to_memory,
};
pub const IMMEDIATE_TO_REGISTER: Description = Description {
    parse_fn: parse_mov_immediate_to_register,
    write_fn: write_immediate_instruction,
};
pub const MEMORY_TO_ACCUMULATOR: Description = Description {
    parse_fn: parse_mov_memory_to_accumulator,
    write_fn: write_typical_instruction,
};
pub const TO_SEGMENT_REGISTER: Description = Description {
    parse_fn: parse_mov_to_segment_register,
    write_fn: write_typical_instruction,
};
