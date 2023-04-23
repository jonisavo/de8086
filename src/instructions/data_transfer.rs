use super::{
    common::{
        create_single_byte_instruction, get_register, parse_typical_instruction,
        write_typical_instruction,
    },
    Description,
};

pub const XCHG_MEMORY_WITH_REGISTER: Description = Description {
    write_fn: write_typical_instruction,
    parse_fn: |bytes| parse_typical_instruction("xchg", bytes, &XCHG_MEMORY_WITH_REGISTER),
};
pub const XCHG_REGISTER_WITH_ACCUMULATOR: Description = Description {
    write_fn: write_typical_instruction,
    parse_fn: |bytes| {
        let register = get_register(bytes[0]);
        let mut instruction =
            create_single_byte_instruction("xchg", &XCHG_REGISTER_WITH_ACCUMULATOR, register);

        // Accumulator is the destination, source is the register
        instruction.fields.direction = false;

        instruction
    },
};
