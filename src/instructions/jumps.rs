use phf::{phf_map, Map};

use crate::{writer::Writer, Instruction};

use super::{
    common::{get_disp_value, InstRegister, InstructionDataFields, InstructionFields, Register},
    Description,
};

pub const CONDITIONAL_JUMP_MNEMONIC_MAP: Map<u8, &'static str> = phf_map! {
    0b01110100u8 => "je",
    0b01111100u8 => "jl",
    0b01111110u8 => "jle",
    0b01110010u8 => "jb",
    0b01110110u8 => "jbe",
    0b01111010u8 => "jp",
    0b01110000u8 => "jo",
    0b01111000u8 => "js",
    0b01110101u8 => "jne",
    0b01111101u8 => "jnl",
    0b01111111u8 => "jnle",
    0b01110011u8 => "jnb",
    0b01110111u8 => "jnbe",
    0b01111011u8 => "jnp",
    0b01110001u8 => "jno",
    0b01111001u8 => "jns",
    0b11100010u8 => "loop",
    0b11100001u8 => "loopz",
    0b11100000u8 => "loopnz",
    0b11100011u8 => "jcxz",
};

fn parse_conditional_jump(bytes: &[u8]) -> Instruction {
    let byte = bytes[0] as u8;
    let mnemonic = CONDITIONAL_JUMP_MNEMONIC_MAP.get(&byte).unwrap();
    let disp = get_disp_value(bytes, 1, 1);

    Instruction {
        mnemonic,
        length: 2,
        fields: InstructionFields::EMPTY,
        register: InstRegister::Reg(Register::AX),
        data_fields: InstructionDataFields::EMPTY,
        disp,
        data: 0,
        description: &CONDITIONAL_JUMP,
    }
}

fn write_conditional_jump(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_jump_displacement(instruction.disp as i8)
        .end_line();
}

pub const CONDITIONAL_JUMP: Description = Description {
    parse_fn: parse_conditional_jump,
    write_fn: write_conditional_jump,
};
