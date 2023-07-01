use phf::{phf_map, Map};

use super::{
    common::{parse_bare_instruction, write_bare_instruction, InstRegister},
    Description,
};

pub const PROCESSOR_CONTROL_MNEMONIC_MAP: Map<u8, &'static str> = phf_map! {
    0b11111000_u8 => "clc",
    0b11110101_u8 => "cmc",
    0b11111001_u8 => "stc",
    0b11111100_u8 => "cld",
    0b11111101_u8 => "std",
    0b11111010_u8 => "cli",
    0b11111011_u8 => "sti",
    0b11110100_u8 => "hlt",
    0b10011011_u8 => "wait",
};

pub const PROCESSOR_CONTROL: Description = Description {
    parse_fn: |bytes, inst| {
        parse_bare_instruction(inst, PROCESSOR_CONTROL_MNEMONIC_MAP[&bytes[0]]);
    },
    write_fn: write_bare_instruction,
};

pub const LOCK: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, "lock"),
    write_fn: |writer, _| {
        writer.set_lock_prefix();
    },
};

pub const SEGMENT_OVERRIDE: Description = Description {
    parse_fn: |bytes, inst| {
        parse_bare_instruction(inst, "segment");
        let seg_reg = (bytes[0] >> 3) & 0b11;
        inst.register = InstRegister::SegReg(seg_reg)
    },
    write_fn: |writer, inst| {
        writer.set_segment_prefix(inst.register.into());
    },
};
