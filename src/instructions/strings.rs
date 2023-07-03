use phf::{phf_map, Map};

use super::{
    common::{parse_bare_instruction, write_bare_instruction},
    opcode::Opcode,
    Description,
};

pub const STRING_MANIPULATION_OPCODE_MAP: Map<u8, Opcode> = phf_map! {
    0b10100100_u8 => Opcode::MOVSB,
    0b10100101_u8 => Opcode::MOVSW,
    0b10100110_u8 => Opcode::CMPSB,
    0b10100111_u8 => Opcode::CMPSW,
    0b10101110_u8 => Opcode::SCASB,
    0b10101111_u8 => Opcode::SCASW,
    0b10101100_u8 => Opcode::LODSB,
    0b10101101_u8 => Opcode::LODSW,
    0b10101010_u8 => Opcode::STOSB,
    0b10101011_u8 => Opcode::STOSW,
};

pub const REPEAT: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, Opcode::REP),
    write_fn: |writer, instruction| {
        writer.set_repeat_prefix(instruction.input[0]);
    },
};

pub const STRING_MANIPULATION: Description = Description {
    parse_fn: |bytes, inst| parse_bare_instruction(inst, STRING_MANIPULATION_OPCODE_MAP[&bytes[0]]),
    write_fn: write_bare_instruction,
};
