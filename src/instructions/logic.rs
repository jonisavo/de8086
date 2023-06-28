use crate::{writer::Writer, Instruction};

use super::{
    arithmetic::parse_immediate_to_accumulator,
    common::{
        get_data_value, get_disp_value, get_displacement_amount, get_register,
        parse_typical_instruction, write_immediate_instruction,
        write_memory_or_register_instruction, write_typical_instruction, InstructionDataFields,
        InstructionFields, RM,
    },
    Description,
};

pub fn write_logic_instruction(writer: &mut Writer, inst: &Instruction) {
    writer.start_instruction(inst);

    if let RM::Eff(_) = inst.data_fields.rm {
        if inst.fields.word {
            writer.write_str("word ");
        } else {
            writer.write_str("byte ");
        }
    }

    let count_str = if inst.fields.shift_rotate { "cl" } else { "1" };

    writer
        .write_str(&inst.address_to_string(inst.data_fields.rm))
        .write_comma_separator()
        .write_str(count_str)
        .end_line();
}

pub const NOT: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "not", bytes),
    write_fn: write_memory_or_register_instruction,
};
pub const SHL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "shl", bytes),
    write_fn: write_logic_instruction,
};
pub const SHR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "shr", bytes),
    write_fn: write_logic_instruction,
};
pub const SAR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "sar", bytes),
    write_fn: write_logic_instruction,
};
pub const ROL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "rol", bytes),
    write_fn: write_logic_instruction,
};
pub const ROR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "ror", bytes),
    write_fn: write_logic_instruction,
};
pub const RCL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "rcl", bytes),
    write_fn: write_logic_instruction,
};
pub const RCR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "rcr", bytes),
    write_fn: write_logic_instruction,
};

pub const AND_WITH_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "and", bytes),
    write_fn: write_typical_instruction,
};
pub const AND_IMMEDIATE_FROM_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, "and", bytes),
    write_fn: write_immediate_instruction,
};

pub const TEST_REGISTER_OR_MEMORY: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "test", bytes),
    write_fn: write_typical_instruction,
};
pub const TEST_IMMEDIATE_AND_REGISTER_OR_MEMORY: Description = Description {
    parse_fn: |bytes, inst| {
        let mut fields = InstructionFields::parse(bytes[0]);
        fields.direction = false;
        let displacement = get_displacement_amount(bytes[1]);
        let has_u16_immediate = fields.word && !fields.sign;
        let immediate_length = has_u16_immediate as u8 + 1;
        let data = get_data_value(bytes, has_u16_immediate, 2 + displacement as usize);
        let register = get_register(bytes[1] >> 3);

        inst.mnemonic = "test";
        inst.length = 2 + displacement + immediate_length;
        inst.fields = fields;
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
            .write_str(&instruction.destination_string())
            .write_comma_separator()
            .write_signed_data(instruction)
            .end_line();
    },
};
pub const TEST_IMMEDIATE_AND_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, "test", bytes),
    write_fn: write_immediate_instruction,
};

pub const OR_WITH_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "or", bytes),
    write_fn: write_typical_instruction,
};
pub const OR_IMMEDIATE_TO_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, "or", bytes),
    write_fn: write_immediate_instruction,
};

pub const XOR_WITH_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "xor", bytes),
    write_fn: write_typical_instruction,
};
pub const XOR_IMMEDIATE_TO_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, "xor", bytes),
    write_fn: write_immediate_instruction,
};
