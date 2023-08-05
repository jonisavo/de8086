use crate::{writer::Writer, Instruction};

use super::{
    arithmetic::parse_immediate_to_accumulator,
    common::{
        get_data_value, get_disp_value, get_displacement_amount, get_register,
        instruction_flags::{self, has_shift_rotate_flag, has_word_flag},
        parse_instruction_flags, parse_typical_instruction, write_immediate_instruction,
        write_memory_or_register_instruction, write_typical_instruction, InstructionDataFields, RM,
    },
    opcode::Opcode,
    Description,
};

pub fn write_logic_instruction(writer: &mut Writer, inst: &Instruction) {
    writer.start_instruction(inst);

    if let RM::Eff(_) = inst.data_fields.rm {
        if has_word_flag(inst.flags) {
            writer.write_str("word ");
        } else {
            writer.write_str("byte ");
        }
    }

    let count_str = if has_shift_rotate_flag(inst.flags) {
        "cl"
    } else {
        "1"
    };

    writer
        .write_rm(inst)
        .write_comma_separator()
        .write_str(count_str)
        .end_line();
}

pub const NOT: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::NOT, bytes),
    write_fn: write_memory_or_register_instruction,
};
pub const SAL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::SAL, bytes),
    write_fn: write_logic_instruction,
};
pub const SHR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::SHR, bytes),
    write_fn: write_logic_instruction,
};
pub const SAR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::SAR, bytes),
    write_fn: write_logic_instruction,
};
pub const ROL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::ROL, bytes),
    write_fn: write_logic_instruction,
};
pub const ROR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::ROR, bytes),
    write_fn: write_logic_instruction,
};
pub const RCL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::RCL, bytes),
    write_fn: write_logic_instruction,
};
pub const RCR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::RCR, bytes),
    write_fn: write_logic_instruction,
};

pub const AND_WITH_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::AND, bytes),
    write_fn: write_typical_instruction,
};
pub const AND_IMMEDIATE_FROM_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, Opcode::AND, bytes),
    write_fn: write_immediate_instruction,
};

pub const TEST_REGISTER_OR_MEMORY: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::TEST, bytes),
    write_fn: write_typical_instruction,
};
pub const TEST_IMMEDIATE_AND_REGISTER_OR_MEMORY: Description = Description {
    parse_fn: |bytes, inst| {
        let mut flags = parse_instruction_flags(bytes[0]);
        flags &= !instruction_flags::DIRECTION;
        let displacement = get_displacement_amount(bytes[1]);
        let has_u16_immediate = has_word_flag(flags);
        let immediate_length = has_u16_immediate as u8 + 1;
        let data = get_data_value(bytes, has_u16_immediate, 2 + displacement as usize);
        let register = get_register(bytes[1] >> 3);

        inst.opcode = Opcode::TEST;
        inst.length = 2 + displacement + immediate_length;
        inst.flags = flags;
        inst.register = register;
        inst.data_fields = InstructionDataFields::parse(bytes[1]);
        inst.disp = get_disp_value(&bytes, displacement, 2);
        inst.data = data;
    },
    write_fn: |writer, instruction| {
        writer.start_instruction(instruction);

        match instruction.data_fields.rm {
            RM::Eff(_) => writer.write_size(instruction),
            RM::Reg(_) => writer,
        };

        writer
            .write_destination(instruction)
            .write_comma_separator()
            .write_signed_data(instruction)
            .end_line();
    },
};
pub const TEST_IMMEDIATE_AND_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, Opcode::TEST, bytes),
    write_fn: write_immediate_instruction,
};

pub const OR_WITH_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::OR, bytes),
    write_fn: write_typical_instruction,
};
pub const OR_IMMEDIATE_TO_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, Opcode::OR, bytes),
    write_fn: write_immediate_instruction,
};

pub const XOR_WITH_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, Opcode::XOR, bytes),
    write_fn: write_typical_instruction,
};
pub const XOR_IMMEDIATE_TO_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, Opcode::XOR, bytes),
    write_fn: write_immediate_instruction,
};
