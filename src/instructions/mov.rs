use crate::{writer::Writer, instructions::common::get_data_value};

use super::{
    common::{
        get_disp_value, get_displacement_amount, get_register, parse_instruction_fields, InstructionDataFields
    },
    description::descriptions::mov::{IMMEDIATE_TO_REGISTER, TO_REGISTER},
    instruction::Instruction, descriptions::mov::IMMEDIATE_TO_MEMORY,
};

pub fn write_mov_to_register(writer: &mut Writer, instruction: &Instruction) {
    writer
        .write(b"mov ")
        .write_string(&instruction.destination_string())
        .write_comma_separator()
        .write_string(&instruction.source_string())
        .end_line();
}

pub fn write_mov_immediate_to_memory(writer: &mut Writer, instruction: &Instruction) {
    writer
        .write(b"mov ")
        .write_string(&instruction.address_to_string(instruction.data_fields.rm))
        .write_comma_separator()
        .write_with_size_specifier(instruction.data, instruction)
        .end_line();
}

pub fn write_mov_immediate_to_register(writer: &mut Writer, instruction: &Instruction) {
    writer
        .write(b"mov ")
        .write_string(&instruction.destination_string())
        .write_comma_separator()
        .write_with_w_flag(instruction.data, instruction)
        .end_line();
}

pub fn parse_mov_to_register(bytes: &[u8]) -> Instruction {
    let displacement = get_displacement_amount(bytes[1]);

    Instruction {
        length: 2 + displacement,
        fields: parse_instruction_fields(bytes[0]),
        register: get_register(bytes[1] >> 3),
        data_fields: InstructionDataFields::parse(bytes[1]),
        disp: get_disp_value(&bytes, displacement, 2),
        data: 0,
        description: &TO_REGISTER,
    }
}

pub fn parse_mov_immediate_to_memory(bytes: &[u8]) -> Instruction {
    let fields = parse_instruction_fields(bytes[0]);
    let displacement = get_displacement_amount(bytes[1]);
    let immediate_length = fields.word as u8 + 1;
    let data = get_data_value(bytes, fields.word, 2 + displacement as usize);

    Instruction {
        length: 2 + displacement + immediate_length,
        fields,
        register: get_register(bytes[1] >> 3),
        data_fields: InstructionDataFields::parse(bytes[1]),
        disp: get_disp_value(&bytes, displacement, 2),
        data,
        description: &IMMEDIATE_TO_MEMORY,
    }
}

pub fn parse_mov_immediate_to_register(bytes: &[u8]) -> Instruction {
    let fields = parse_instruction_fields(bytes[0] >> 3);
    let length = fields.word as u8 + 2;
    let data = get_data_value(bytes, fields.word, 1);

    Instruction {
        length,
        fields,
        register: get_register(bytes[0]),
        data_fields: InstructionDataFields::EMPTY,
        disp: 0,
        data,
        description: &IMMEDIATE_TO_REGISTER,
    }
}
