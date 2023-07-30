use crate::{writer::Writer, Instruction};

use super::{
    common::parse_typical_instruction,
    common::{get_disp_value, parse_bare_instruction, write_bare_instruction},
    opcode::Opcode,
    Description,
};

fn parse_direct_within_segment(bytes: &[u8], inst: &mut Instruction) {
    inst.length = 3;
    inst.disp = get_disp_value(bytes, 2, 1);
}

fn write_direct_within_segment(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_jump_displacement(instruction.disp)
        .end_line();
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
    inst.opcode = match bytes[0] {
        0b01110100_u8 => Opcode::JE,
        0b01111100_u8 => Opcode::JL,
        0b01111110_u8 => Opcode::JLE,
        0b01110010_u8 => Opcode::JB,
        0b01110110_u8 => Opcode::JBE,
        0b01111010_u8 => Opcode::JP,
        0b01110000_u8 => Opcode::JO,
        0b01111000_u8 => Opcode::JS,
        0b01110101_u8 => Opcode::JNE,
        0b01111101_u8 => Opcode::JGE,
        0b01111111_u8 => Opcode::JG,
        0b01110011_u8 => Opcode::JAE,
        0b01110111_u8 => Opcode::JA,
        0b01111011_u8 => Opcode::JNP,
        0b01110001_u8 => Opcode::JNO,
        0b01111001_u8 => Opcode::JNS,
        0b11100010_u8 => Opcode::LOOP,
        0b11100001_u8 => Opcode::LOOPE,
        0b11100000_u8 => Opcode::LOOPNE,
        0b11100011_u8 => Opcode::JCXZ,
        b => unreachable!("Invalid conditional jump opcode: {:b}", b),
    };
    inst.length = 2;
    inst.disp = get_disp_value(bytes, 1, 1);
}

fn write_conditional_jump(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_jump_displacement(instruction.disp)
        .end_line();
}

const CONTROL_TRANSFER_OPCODES: [Opcode; 2] = [Opcode::CALL, Opcode::JMP];

pub const DIRECT_WITHIN_SEGMENT: Description = Description {
    parse_fn: |bytes, inst| {
        parse_direct_within_segment(bytes, inst);
        inst.opcode = CONTROL_TRANSFER_OPCODES[(bytes[0] & 0b1) as usize];
    },
    write_fn: write_direct_within_segment,
};

pub const JUMP_DIRECT_WITHIN_SEGMENT_SHORT: Description = Description {
    parse_fn: |bytes, inst| {
        inst.length = 2;
        inst.disp = get_disp_value(bytes, 1, 1);
        inst.opcode = Opcode::JMP;
    },
    write_fn: write_direct_within_segment,
};

pub const INDIRECT_WITHIN_SEGMENT: Description = Description {
    parse_fn: |bytes, inst| {
        let sixth_bit_set = ((bytes[1] >> 5) & 0b1) == 0b1;
        let opcode = CONTROL_TRANSFER_OPCODES[sixth_bit_set as usize];
        parse_typical_instruction(inst, opcode, bytes);
    },
    write_fn: |writer, inst| {
        writer.start_instruction(inst).write_rm(inst).end_line();
    },
};

pub const DIRECT_INTERSEGMENT: Description = Description {
    parse_fn: |bytes, inst| {
        let three_high_bits_set = (bytes[0] >> 5) == 0b111;
        let opcode = CONTROL_TRANSFER_OPCODES[three_high_bits_set as usize];
        inst.opcode = opcode;
        inst.length = 5;
    },
    write_fn: write_direct_intersegment,
};

pub const INDIRECT_INTERSEGMENT: Description = Description {
    parse_fn: |bytes, inst| {
        let sixth_bit_set = ((bytes[1] >> 5) & 0b1) == 0b1;
        let opcode = CONTROL_TRANSFER_OPCODES[sixth_bit_set as usize];
        parse_typical_instruction(inst, opcode, bytes);
    },
    write_fn: |writer, inst| {
        writer
            .start_instruction(inst)
            .write_str("far ")
            .write_rm(inst)
            .end_line();
    },
};

pub const CONDITIONAL_JUMP: Description = Description {
    parse_fn: parse_conditional_jump,
    write_fn: write_conditional_jump,
};

const RET_OPCODES: [Opcode; 2] = [Opcode::RET, Opcode::RETF];

fn get_ret_opcode(bytes: &[u8]) -> Opcode {
    let fourth_bit_set = ((bytes[0] >> 3) & 0b1) == 0b1;
    RET_OPCODES[fourth_bit_set as usize]
}

pub const RETURN_NO_VALUE: Description = Description {
    parse_fn: |bytes, inst| parse_bare_instruction(inst, get_ret_opcode(bytes)),
    write_fn: write_bare_instruction,
};
pub const RETURN_WITH_VALUE: Description = Description {
    parse_fn: |bytes, inst| {
        inst.length = 3;
        inst.opcode = get_ret_opcode(bytes);
        inst.disp = get_disp_value(bytes, 2, 1);
    },
    write_fn: |writer, inst| {
        writer
            .start_instruction(inst)
            .write_str(inst.disp.to_string().as_str())
            .end_line();
    },
};

const INTERRUPT_OPCODES: [Opcode; 4] = [Opcode::INT3, Opcode::INT, Opcode::INTO, Opcode::IRET];

pub const INTERRUPT: Description = Description {
    parse_fn: |bytes, inst| {
        let interrupt_opcode = bytes[0] & 0b11;
        let has_data = interrupt_opcode == 1;

        inst.opcode = INTERRUPT_OPCODES[interrupt_opcode as usize];
        inst.length = 1 + has_data as u8;
        inst.data = has_data as u16 * bytes[has_data as usize] as u16;
    },
    write_fn: |writer, inst| {
        writer.start_instruction(inst);

        if (inst.data as u8) != 0 {
            writer.write_str(inst.data.to_string().as_str());
        }

        writer.end_line();
    },
};
