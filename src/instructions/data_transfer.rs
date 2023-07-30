use phf::{phf_map, Map};

use crate::{writer::Writer, Instruction};

use super::{
    common::{
        create_single_byte_instruction, get_register, instruction_flags, parse_instruction_flags,
        parse_typical_instruction, register, write_bare_instruction, write_typical_instruction,
        InstRegister, WORD_REGISTER_STRINGS,
    },
    opcode::Opcode,
    Description,
};

pub const DATA_TRANSFER_OPCODE_MAP: Map<u8, Opcode> = phf_map! {
    0b11010111_u8 => Opcode::XLAT,
    0b10001101_u8 => Opcode::LEA,
    0b11000101_u8 => Opcode::LDS,
    0b11000100_u8 => Opcode::LES,
    0b10011111_u8 => Opcode::LAHF,
    0b10011110_u8 => Opcode::SAHF,
    0b10011100_u8 => Opcode::PUSHF,
    0b10011101_u8 => Opcode::POPF,
};

pub const XCHG_MEMORY_WITH_REGISTER: Description = Description {
    write_fn: write_typical_instruction,
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::XCHG, bytes),
};
pub const XCHG_REGISTER_WITH_ACCUMULATOR: Description = Description {
    write_fn: write_typical_instruction,
    parse_fn: |bytes, inst| {
        let register = get_register(bytes[0]);

        create_single_byte_instruction(inst, Opcode::XCHG, register);

        // Accumulator is the destination, source is the register
        inst.flags |= instruction_flags::DIRECTION;
    },
};

pub fn write_in_out_fixed_port(writer: &mut Writer, instruction: &Instruction) {
    let destination_string = writer.address_to_string(instruction, instruction.get_destination());
    let data_string = instruction.data.to_string();
    let op1 = if instruction.opcode == Opcode::IN {
        destination_string.as_str()
    } else {
        data_string.as_str()
    };
    let op2 = if instruction.opcode == Opcode::IN {
        data_string.as_str()
    } else {
        destination_string.as_str()
    };

    writer
        .start_instruction(instruction)
        .write_str(op1)
        .write_comma_separator()
        .write_str(op2)
        .end_line();
}

pub fn write_in_out_variable_port(writer: &mut Writer, instruction: &Instruction) {
    const DX_STR: &str = &WORD_REGISTER_STRINGS[register::DX as usize];
    let destination_string = &writer.address_to_string(instruction, instruction.get_destination());

    let op1 = if instruction.opcode == Opcode::IN {
        destination_string
    } else {
        DX_STR
    };
    let op2 = if instruction.opcode == Opcode::IN {
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

fn get_in_or_out_opcode(byte: u8) -> Opcode {
    let second_bit = byte >> 1 & 0b1;

    if second_bit == 0 {
        Opcode::IN
    } else {
        Opcode::OUT
    }
}

pub fn parse_in_out_fixed_port(bytes: &[u8], inst: &mut Instruction) {
    inst.opcode = get_in_or_out_opcode(bytes[0]);
    inst.length = 2;
    inst.flags = parse_instruction_flags(bytes[0]);
    inst.register = InstRegister::Reg(register::AX);
    inst.data = bytes[1] as u16;
}

pub fn parse_in_out_variable_port(bytes: &[u8], inst: &mut Instruction) {
    inst.opcode = get_in_or_out_opcode(bytes[0]);
    inst.length = 1;
    inst.flags = parse_instruction_flags(bytes[0]);
    inst.register = InstRegister::Reg(register::AX);
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
        let opcode = DATA_TRANSFER_OPCODE_MAP[&bytes[0]];
        parse_typical_instruction(inst, opcode, bytes);
        inst.flags |= instruction_flags::WORD;
        inst.flags |= instruction_flags::DIRECTION;
    },
    write_fn: write_typical_instruction,
};

pub const OTHER_DATA_TRANSFER: Description = Description {
    parse_fn: |bytes, inst| {
        inst.opcode = DATA_TRANSFER_OPCODE_MAP[&bytes[0]];
        inst.length = 1;
    },
    write_fn: write_bare_instruction,
};
