use crate::{writer::Writer, Instruction};

use super::{
    common::{parse_typical_instruction, write_memory_or_register_instruction, RM},
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
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "not", bytes, &NOT),
    write_fn: write_memory_or_register_instruction,
};
pub const SHL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "shl", bytes, &SHL),
    write_fn: write_logic_instruction,
};
pub const SHR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "shr", bytes, &SHR),
    write_fn: write_logic_instruction,
};
pub const SAR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "sar", bytes, &SAR),
    write_fn: write_logic_instruction,
};
pub const ROL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "rol", bytes, &ROL),
    write_fn: write_logic_instruction,
};
pub const ROR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "ror", bytes, &ROR),
    write_fn: write_logic_instruction,
};
pub const RCL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "rcl", bytes, &RCL),
    write_fn: write_logic_instruction,
};
pub const RCR: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "rcr", bytes, &RCR),
    write_fn: write_logic_instruction,
};
