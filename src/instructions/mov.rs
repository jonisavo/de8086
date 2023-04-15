use crate::writer::Writer;

use super::{
    common::{
        get_data_value, get_disp_value, get_displacement_amount, get_register,
        parse_typical_instruction, write_immediate_instruction, write_typical_instruction,
        InstructionDataFields, InstructionFields, Register,
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

pub fn parse_mov_immediate_to_memory(bytes: &[u8]) -> Instruction {
    let fields = InstructionFields::parse(bytes[0]);
    let displacement = get_displacement_amount(bytes[1]);
    let immediate_length = fields.word as u8 + 1;
    let data = get_data_value(bytes, fields.word, 2 + displacement as usize);

    Instruction {
        mnemonic: "mov",
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
    let fields = InstructionFields::parse(bytes[0] >> 3);
    let length = fields.word as u8 + 2;
    let data = get_data_value(bytes, fields.word, 1);

    Instruction {
        mnemonic: "mov",
        length,
        fields,
        register: get_register(bytes[0]),
        data_fields: InstructionDataFields::EMPTY,
        disp: 0,
        data,
        description: &IMMEDIATE_TO_REGISTER,
    }
}

pub fn parse_mov_memory_to_accumulator(bytes: &[u8]) -> Instruction {
    let mut fields = InstructionFields::parse(bytes[0]);
    fields.direction = !fields.direction;
    let disp = get_disp_value(bytes, 2, 1);

    Instruction {
        mnemonic: "mov",
        length: 3,
        fields,
        register: Register::AX,
        data_fields: InstructionDataFields::DIRECT_ADDRESS,
        disp,
        data: 0,
        description: &MEMORY_TO_ACCUMULATOR,
    }
}

pub const TO_REGISTER: Description = Description {
    parse_fn: |b| parse_typical_instruction("mov", b, &TO_REGISTER),
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
