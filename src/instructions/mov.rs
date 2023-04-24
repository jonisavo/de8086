use crate::writer::Writer;

use super::{
    common::{
        get_data_value, get_disp_value, get_displacement_amount, get_register,
        get_segment_register, parse_typical_instruction, write_immediate_instruction,
        write_typical_instruction, InstRegister, InstructionDataFields, InstructionFields,
        Register,
    },
    instruction::Instruction,
    Description,
};

pub fn write_mov_immediate_to_memory(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_str(&instruction.address_to_string(instruction.data_fields.rm))
        .write_comma_separator()
        .write_with_size(instruction.data, instruction)
        .end_line();
}

pub fn parse_mov_immediate_to_memory(bytes: &[u8], inst: &mut Instruction) {
    let fields = InstructionFields::parse(bytes[0]);
    let displacement = get_displacement_amount(bytes[1]);
    let immediate_length = fields.word as u8 + 1;
    let data = get_data_value(bytes, fields.word, 2 + displacement as usize);

    inst.mnemonic = "mov";
    inst.length = 2 + displacement + immediate_length;
    inst.fields = fields;
    inst.register = get_register(bytes[1] >> 3);
    inst.data_fields = InstructionDataFields::parse(bytes[1]);
    inst.disp = get_disp_value(&bytes, displacement, 2);
    inst.data = data;
    inst.description = &IMMEDIATE_TO_MEMORY;
}

pub fn parse_mov_immediate_to_register(bytes: &[u8], inst: &mut Instruction) {
    let fields = InstructionFields::parse(bytes[0] >> 3);
    let length = fields.word as u8 + 2;
    let data = get_data_value(bytes, fields.word, 1);

    inst.mnemonic = "mov";
    inst.length = length;
    inst.fields = fields;
    inst.register = get_register(bytes[0]);
    inst.data = data;
    inst.description = &IMMEDIATE_TO_REGISTER;
}

pub fn parse_mov_memory_to_accumulator(bytes: &[u8], inst: &mut Instruction) {
    let mut fields = InstructionFields::parse(bytes[0]);
    fields.direction = !fields.direction;

    inst.mnemonic = "mov";
    inst.length = 3;
    inst.fields = fields;
    inst.register = InstRegister::Reg(Register::AX);
    inst.data_fields = InstructionDataFields::DIRECT_ADDRESS;
    inst.disp = get_disp_value(bytes, 2, 1);
    inst.description = &MEMORY_TO_ACCUMULATOR;
}

pub fn parse_mov_to_segment_register(bytes: &[u8], inst: &mut Instruction) {
    parse_typical_instruction(inst, "mov", bytes, &TO_SEGMENT_REGISTER);

    inst.register = get_segment_register(bytes[1] >> 3);
    inst.fields.word = true;
}

pub const TO_REGISTER: Description = Description {
    parse_fn: |b, inst| parse_typical_instruction(inst, "mov", b, &TO_REGISTER),
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const IMMEDIATE_TO_MEMORY: Description = Description {
    parse_fn: parse_mov_immediate_to_memory,
    write_fn: write_mov_immediate_to_memory,
};
pub const IMMEDIATE_TO_REGISTER: Description = Description {
    parse_fn: parse_mov_immediate_to_register,
    write_fn: |writer, inst| write_immediate_instruction(writer, inst),
};
pub const MEMORY_TO_ACCUMULATOR: Description = Description {
    parse_fn: parse_mov_memory_to_accumulator,
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const TO_SEGMENT_REGISTER: Description = Description {
    parse_fn: parse_mov_to_segment_register,
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
