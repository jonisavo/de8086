use phf::{phf_map, Map};

use crate::{writer::Writer, Instruction};

use super::{
    common::{
        create_single_byte_instruction, get_register, parse_typical_instruction, register,
        write_bare_instruction, write_typical_instruction, InstRegister, InstructionFields,
        WORD_REGISTER_STRINGS,
    },
    Description,
};

pub const DATA_TRANSFER_MNEMONIC_MAP: Map<u8, &'static str> = phf_map! {
    0b11010111u8 => "xlat",
    0b10001101u8 => "lea",
    0b11000101u8 => "lds",
    0b11000100u8 => "les",
    0b10011111u8 => "lahf",
    0b10011110u8 => "sahf",
    0b10011100u8 => "pushf",
    0b10011101u8 => "popf",
};

pub const XCHG_MEMORY_WITH_REGISTER: Description = Description {
    write_fn: write_typical_instruction,
    parse_fn: |bytes, inst| {
        parse_typical_instruction(inst, "xchg", bytes, &XCHG_MEMORY_WITH_REGISTER)
    },
};
pub const XCHG_REGISTER_WITH_ACCUMULATOR: Description = Description {
    write_fn: write_typical_instruction,
    parse_fn: |bytes, inst| {
        let register = get_register(bytes[0]);

        create_single_byte_instruction(inst, "xchg", &XCHG_REGISTER_WITH_ACCUMULATOR, register);

        // Accumulator is the destination, source is the register
        inst.fields.direction = false;
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
    const DX_STR: &str = &WORD_REGISTER_STRINGS[register::DX as usize];
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

pub fn parse_in_out_fixed_port(bytes: &[u8], inst: &mut Instruction) {
    let second_bit = bytes[0] >> 1 & 0b1;
    let mnemonic = if second_bit == 0 { "in" } else { "out" };

    inst.mnemonic = mnemonic;
    inst.length = 2;
    inst.fields = InstructionFields::parse(bytes[0]);
    inst.register = InstRegister::Reg(register::AX);
    inst.data = bytes[1] as u16;
    inst.description = &IN_OUT_FIXED_PORT;
}

pub fn parse_in_out_variable_port(bytes: &[u8], inst: &mut Instruction) {
    let second_bit = bytes[0] >> 1 & 0b1;
    let mnemonic = if second_bit == 0 { "in" } else { "out" };

    inst.mnemonic = mnemonic;
    inst.length = 1;
    inst.fields = InstructionFields::parse(bytes[0]);
    inst.register = InstRegister::Reg(register::AX);
    inst.description = &IN_OUT_VARIABLE_PORT;
}

pub const IN_OUT_FIXED_PORT: Description = Description {
    parse_fn: parse_in_out_fixed_port,
    write_fn: write_in_out_fixed_port,
};
pub const IN_OUT_VARIABLE_PORT: Description = Description {
    parse_fn: parse_in_out_variable_port,
    write_fn: write_in_out_variable_port,
};

pub const LEA_LDS_LES: Description = Description {
    parse_fn: |bytes, inst| {
        let mnemonic = DATA_TRANSFER_MNEMONIC_MAP.get(&bytes[0]).unwrap();
        parse_typical_instruction(inst, mnemonic, bytes, &LEA_LDS_LES);
        inst.fields.direction = true;
        inst.fields.word = true;
    },
    write_fn: write_typical_instruction,
};

pub const OTHER_DATA_TRANSFER: Description = Description {
    parse_fn: |bytes, inst| {
        let mnemonic = DATA_TRANSFER_MNEMONIC_MAP.get(&bytes[0]).unwrap();
        inst.mnemonic = mnemonic;
        inst.length = 1;
        inst.description = &OTHER_DATA_TRANSFER;
    },
    write_fn: write_bare_instruction,
};
