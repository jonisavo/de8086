use phf::{phf_map, Map};

use crate::{writer::Writer, Instruction};

use super::{common::get_disp_value, common::parse_typical_instruction, Description};

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

fn parse_direct_within_segment(bytes: &[u8], inst: &mut Instruction) {
    inst.length = 3;
    inst.disp = get_disp_value(bytes, 2, 1);
}

fn write_direct_within_segment(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_str(instruction.disp.to_string().as_str())
        .end_line();
}

fn parse_direct_intersegment(_bytes: &[u8], inst: &mut Instruction) {
    inst.length = 5;
}

fn write_direct_intersegment(writer: &mut Writer, instruction: &Instruction) {
    let ip = (instruction.input[2] as u16) << 8 | instruction.input[1] as u16;
    let cs = (instruction.input[4] as u16) << 8 | instruction.input[3] as u16;

    writer
        .start_instruction(instruction)
        .write_str(cs.to_string().as_str())
        .write_byte(b':')
        .write_str(ip.to_string().as_str())
        .end_line();
}

fn parse_conditional_jump(bytes: &[u8], inst: &mut Instruction) {
    let byte = bytes[0] as u8;

    inst.mnemonic = CONDITIONAL_JUMP_MNEMONIC_MAP.get(&byte).unwrap();
    inst.length = 2;
    inst.disp = get_disp_value(bytes, 1, 1);
}

fn write_conditional_jump(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_jump_displacement(instruction.disp as i8)
        .end_line();
}

pub const CALL_DIRECT_WITHIN_SEGMENT: Description = Description {
    parse_fn: |bytes, inst| {
        inst.mnemonic = "call";
        parse_direct_within_segment(bytes, inst);
    },
    write_fn: write_direct_within_segment,
};

pub const CALL_INDIRECT_WITHIN_SEGMENT: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "call", bytes),
    write_fn: |writer, inst| {
        writer
            .start_instruction(inst)
            .write_str(&inst.address_to_string(inst.data_fields.rm))
            .end_line();
    },
};

pub const CALL_DIRECT_INTERSEGMENT: Description = Description {
    parse_fn: |bytes, inst| {
        inst.mnemonic = "call";
        parse_direct_intersegment(bytes, inst);
    },
    write_fn: write_direct_intersegment,
};

pub const CALL_INDIRECT_INTERSEGMENT: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "call", bytes),
    write_fn: |writer, inst| {
        writer
            .start_instruction(inst)
            .write_str("far ")
            .write_str(&inst.address_to_string(inst.data_fields.rm))
            .end_line();
    },
};

pub const CONDITIONAL_JUMP: Description = Description {
    parse_fn: parse_conditional_jump,
    write_fn: write_conditional_jump,
};
