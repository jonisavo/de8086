use crate::{writer::Writer, Instruction};

use super::{
    common::{
        create_single_byte_instruction, get_register, parse_typical_instruction,
        write_typical_instruction, InstRegister, InstructionDataFields, InstructionFields,
        Register, WORD_REGISTER_STRINGS,
    },
    Description,
};

pub const XCHG_MEMORY_WITH_REGISTER: Description = Description {
    write_fn: write_typical_instruction,
    parse_fn: |bytes| parse_typical_instruction("xchg", bytes, &XCHG_MEMORY_WITH_REGISTER),
};
pub const XCHG_REGISTER_WITH_ACCUMULATOR: Description = Description {
    write_fn: write_typical_instruction,
    parse_fn: |bytes| {
        let register = get_register(bytes[0]);
        let mut instruction =
            create_single_byte_instruction("xchg", &XCHG_REGISTER_WITH_ACCUMULATOR, register);

        // Accumulator is the destination, source is the register
        instruction.fields.direction = false;

        instruction
    },
};

pub fn write_in_out_fixed_port(writer: &mut Writer, instruction: &Instruction) {
    let op1 = if instruction.mnemonic == "in" {
        instruction.destination_string()
    } else {
        instruction.data.to_string()
    };
    let op2 = if instruction.mnemonic == "in" {
        instruction.data.to_string()
    } else {
        instruction.destination_string()
    };

    writer
        .start_instruction(instruction)
        .write_str(&op1)
        .write_comma_separator()
        .write_str(&op2)
        .end_line();
}

pub fn write_in_out_variable_port(writer: &mut Writer, instruction: &Instruction) {
    const DX_STR: &str = &WORD_REGISTER_STRINGS[Register::DX as usize];
    let destination_string = &instruction.destination_string();

    let op1 = if instruction.mnemonic == "in" {
        destination_string
    } else {
        DX_STR
    };
    let op2 = if instruction.mnemonic == "in" {
        DX_STR
    } else {
        destination_string
    };

    writer
        .start_instruction(instruction)
        .write_str(&op1)
        .write_comma_separator()
        .write_str(&op2)
        .end_line();
}

pub fn parse_in_out_fixed_port(bytes: &[u8]) -> Instruction {
    let second_bit = bytes[0] >> 1 & 0b1;
    let mnemonic = if second_bit == 0 { "in" } else { "out" };

    Instruction {
        mnemonic,
        length: 2,
        fields: InstructionFields::parse(bytes[0]),
        register: InstRegister::Reg(Register::AX),
        data_fields: InstructionDataFields::EMPTY,
        disp: 0,
        data: bytes[1] as u16,
        description: &IN_OUT_FIXED_PORT,
    }
}

pub fn parse_in_out_variable_port(bytes: &[u8]) -> Instruction {
    let second_bit = bytes[0] >> 1 & 0b1;
    let mnemonic = if second_bit == 0 { "in" } else { "out" };

    Instruction {
        mnemonic,
        length: 1,
        fields: InstructionFields::parse(bytes[0]),
        register: InstRegister::Reg(Register::AX),
        data_fields: InstructionDataFields::EMPTY,
        disp: 0,
        data: 0,
        description: &IN_OUT_VARIABLE_PORT,
    }
}

pub const IN_OUT_FIXED_PORT: Description = Description {
    write_fn: write_in_out_fixed_port,
    parse_fn: parse_in_out_fixed_port,
};
pub const IN_OUT_VARIABLE_PORT: Description = Description {
    write_fn: write_in_out_variable_port,
    parse_fn: parse_in_out_variable_port,
};
