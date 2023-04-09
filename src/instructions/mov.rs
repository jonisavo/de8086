use std::io::{BufWriter, Stdout, Write};

use super::{
    common::{
        get_disp_value, get_displacement_amount, get_register, parse_instruction_fields,
        InstructionData,
    },
    description::descriptions::mov::TO_REGISTER,
    instruction::Instruction,
};

pub fn write_mov_to_register(writer: &mut BufWriter<Stdout>, instruction: &Instruction) {
    writer.write_all(b"mov ").unwrap();
    writer
        .write_all(instruction.destination_string().as_bytes())
        .unwrap();
    writer.write_all(b", ").unwrap();
    writer
        .write_all(instruction.source_string().as_bytes())
        .unwrap();
    writer.write_all(b"\n").unwrap();
}

pub fn parse_mov_to_register(bytes: &[u8]) -> Option<Instruction> {
    let displacement = get_displacement_amount(bytes[1]);

    Some(Instruction {
        length: 2 + displacement,
        fields: parse_instruction_fields(bytes[0]),
        register: get_register(bytes[1] >> 3),
        data: InstructionData::parse_fields(bytes[1]),
        disp: get_disp_value(&bytes, displacement, 2),
        additional_data: 0,
        description: &TO_REGISTER,
    })
}
