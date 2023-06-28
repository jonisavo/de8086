use phf::{phf_map, Map};

use super::{
    common::{parse_bare_instruction, write_bare_instruction},
    Description,
};

pub const STRING_MANIPULATION_MNEMONIC_MAP: Map<u8, &'static str> = phf_map! {
    0b10100100_u8 => "movsb",
    0b10100101_u8 => "movsw",
    0b10100110_u8 => "cmpsb",
    0b10100111_u8 => "cmpsw",
    0b10101110_u8 => "scasb",
    0b10101111_u8 => "scasw",
    0b10101100_u8 => "lodsb",
    0b10101101_u8 => "lodsw",
    0b10101010_u8 => "stosb",
    0b10101011_u8 => "stosw",
};

pub const REPEAT: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, "rep"),
    write_fn: |writer, instruction| {
        writer.set_repeat_prefix(instruction.input[0]);
    },
};

pub const STRING_MANIPULATION: Description = Description {
    parse_fn: |bytes, inst| {
        parse_bare_instruction(inst, STRING_MANIPULATION_MNEMONIC_MAP[&bytes[0]])
    },
    write_fn: write_bare_instruction,
};
