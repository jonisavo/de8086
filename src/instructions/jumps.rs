use phf::{phf_map, Map};

use crate::{writer::Writer, Instruction};

use super::{common::get_disp_value, Description};

pub const CONDITIONAL_JUMP_MNEMONIC_MAP: Map<u8, &'static str> = phf_map! {
    0b01110100_u8 => "je",
    0b01111100_u8 => "jl",
    0b01111110_u8 => "jle",
    0b01110010_u8 => "jb",
    0b01110110_u8 => "jbe",
    0b01111010_u8 => "jp",
    0b01110000_u8 => "jo",
    0b01111000_u8 => "js",
    0b01110101_u8 => "jne",
    0b01111101_u8 => "jnl",
    0b01111111_u8 => "jnle",
    0b01110011_u8 => "jnb",
    0b01110111_u8 => "jnbe",
    0b01111011_u8 => "jnp",
    0b01110001_u8 => "jno",
    0b01111001_u8 => "jns",
    0b11100010_u8 => "loop",
    0b11100001_u8 => "loopz",
    0b11100000_u8 => "loopnz",
    0b11100011_u8 => "jcxz",
};

fn parse_conditional_jump(bytes: &[u8], inst: &mut Instruction) {
    let byte = bytes[0] as u8;

    inst.mnemonic = CONDITIONAL_JUMP_MNEMONIC_MAP.get(&byte).unwrap();
    inst.length = 2;
    inst.disp = get_disp_value(bytes, 1, 1);
    inst.description = &CONDITIONAL_JUMP;
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
