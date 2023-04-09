use crate::writer::Writer;

use super::{
    common::{
        get_disp_value, get_displacement_amount, get_register, parse_instruction_fields,
        InstructionData,
    },
    description::descriptions::mov::{IMMEDIATE_TO_REGISTER, TO_REGISTER},
    instruction::Instruction,
};

pub fn write_mov_to_register(writer: &mut Writer, instruction: &Instruction) {
    writer
        .write(b"mov ")
        .write_string(&instruction.destination_string())
        .write_comma_separator()
        .write_string(&instruction.source_string())
        .end_line();
}

pub fn write_mov_immediate_to_register(writer: &mut Writer, instruction: &Instruction) {
    writer
        .write(b"mov ")
        .write_string(&instruction.destination_string())
        .write_comma_separator()
        .write(format!("{:#004x}", instruction.disp).as_bytes())
        .end_line();
}

pub fn parse_mov_to_register(bytes: &[u8]) -> Instruction {
    let displacement = get_displacement_amount(bytes[1]);

    Instruction {
        length: 2 + displacement,
        fields: parse_instruction_fields(bytes[0]),
        register: get_register(bytes[1] >> 3),
        data: InstructionData::parse_fields(bytes[1]),
        disp: get_disp_value(&bytes, displacement, 2),
        additional_data: 0,
        description: &TO_REGISTER,
    }
}

pub fn parse_mov_immediate_to_register(bytes: &[u8]) -> Instruction {
    let fields = parse_instruction_fields(bytes[0] >> 3);
    let length = if fields.word { 3 } else { 2 };
    let disp = get_disp_value(bytes, length - 1, 1);

    Instruction {
        length,
        fields,
        register: get_register(bytes[0]),
        data: InstructionData::Data(0),
        disp,
        additional_data: 0,
        description: &IMMEDIATE_TO_REGISTER,
    }
}
