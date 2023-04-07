use std::io::{BufWriter, Write};

use crate::instructions::common::{Instruction, InstructionHeader};

use super::common::parse_instruction_data_fields;

pub fn write_mov<T: std::io::Write>(writer: &mut BufWriter<T>, instruction: &Instruction) {
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

pub fn parse_mov(header: InstructionHeader, data: u16) -> Instruction {
    let data = parse_instruction_data_fields(data);

    Instruction {
        header,
        data,
        disp: 0,
        additional_data: 0,
    }
}
