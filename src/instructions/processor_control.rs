use super::{
    common::{parse_bare_instruction, write_bare_instruction, InstRegister},
    opcode::Opcode,
    Description,
};

pub const PROCESSOR_CONTROL: Description = Description {
    parse_fn: |bytes, inst| {
        let opcode = match bytes[0] {
            0b11111000_u8 => Opcode::CLC,
            0b11110101_u8 => Opcode::CMC,
            0b11111001_u8 => Opcode::STC,
            0b11111100_u8 => Opcode::CLD,
            0b11111101_u8 => Opcode::STD,
            0b11111010_u8 => Opcode::CLI,
            0b11111011_u8 => Opcode::STI,
            0b11110100_u8 => Opcode::HLT,
            0b10011011_u8 => Opcode::WAIT,
            _ => unreachable!("Invalid processor control opcode"),
        };
        parse_bare_instruction(inst, opcode);
    },
    write_fn: write_bare_instruction,
};

pub const LOCK: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, Opcode::LOCK),
    write_fn: |writer, _| {
        writer.set_lock_prefix();
    },
};

pub const SEGMENT_OVERRIDE: Description = Description {
    parse_fn: |bytes, inst| {
        parse_bare_instruction(inst, Opcode::SEGMENT);
        let seg_reg = (bytes[0] >> 3) & 0b11;
        inst.register = InstRegister::SegReg(seg_reg)
    },
    write_fn: |writer, inst| {
        writer.set_segment_prefix(inst.register.into());
    },
};

pub const NOP: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, Opcode::NOP),
    write_fn: |writer, inst| write_bare_instruction(writer, inst),
};
