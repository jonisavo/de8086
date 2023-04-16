use crate::{writer::Writer, Instruction};

use super::{
    common::{
        get_data_value, get_disp_value, get_displacement_amount, get_register, InstRegister,
        InstructionDataFields, InstructionFields, Register,
    },
    Description,
};

pub fn parse_arithmetic_imm_to_register_memory(bytes: &[u8]) -> Instruction {
    let fields = InstructionFields::parse(bytes[0]);
    let displacement = get_displacement_amount(bytes[1]);
    let immediate_length = (fields.word && !fields.sign) as u8 + 1;
    let data = get_data_value(
        bytes,
        fields.word && !fields.sign,
        2 + displacement as usize,
    );
    let register = get_register(bytes[1] >> 3);
    let register_number = <InstRegister as Into<Register>>::into(register) as usize;
    let mnemonic = ARITHMETIC_MNEMONICS[register_number];

    Instruction {
        mnemonic,
        length: 2 + displacement + immediate_length,
        fields,
        register,
        data_fields: InstructionDataFields::parse(bytes[1]),
        disp: get_disp_value(&bytes, displacement, 2),
        data,
        description: &IMMEDIATE_TO_REGISTER_MEMORY,
    }
}

pub const ARITHMETIC_MNEMONICS: [&str; 8] = ["add", "or", "adc", "sbb", "and", "sub", "xor", "cmp"];

pub fn write_arithmetic_imm_to_register_memory(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_str(&instruction.address_to_string(instruction.data_fields.rm))
        .write_comma_separator()
        .write_with_size(instruction.data, instruction)
        .end_line();
}

pub fn parse_immediate_to_accumulator(
    mnemonic: &'static str,
    bytes: &[u8],
    description: &'static Description,
) -> Instruction {
    let fields = InstructionFields::parse(bytes[0]);
    let length = fields.word as u8 + 2;
    let data = get_data_value(bytes, fields.word, 1);

    Instruction {
        mnemonic,
        length,
        fields,
        register: InstRegister::Reg(Register::AX),
        data_fields: InstructionDataFields::EMPTY,
        disp: 0,
        data,
        description,
    }
}

pub mod add {
    use crate::instructions::{
        common::{
            parse_typical_instruction, write_immediate_instruction, write_typical_instruction,
        },
        Description,
    };

    use super::parse_immediate_to_accumulator;

    pub const TO_REGISTER: Description = Description {
        parse_fn: |b| parse_typical_instruction("add", b, &TO_REGISTER),
        write_fn: |writer, inst| write_typical_instruction(writer, inst),
    };
    pub const IMMEDIATE_TO_ACCUMULATOR: Description = Description {
        parse_fn: |bytes| parse_immediate_to_accumulator("add", bytes, &IMMEDIATE_TO_ACCUMULATOR),
        write_fn: |writer, inst| write_immediate_instruction(writer, inst),
    };
}

pub mod sub {
    use crate::instructions::{
        common::{
            parse_typical_instruction, write_immediate_instruction, write_typical_instruction,
        },
        Description,
    };

    use super::parse_immediate_to_accumulator;

    pub const TO_REGISTER: Description = Description {
        parse_fn: |b| parse_typical_instruction("sub", b, &TO_REGISTER),
        write_fn: |writer, inst| write_typical_instruction(writer, inst),
    };
    pub const IMMEDIATE_FROM_ACCUMULATOR: Description = Description {
        parse_fn: |bytes| parse_immediate_to_accumulator("sub", bytes, &IMMEDIATE_FROM_ACCUMULATOR),
        write_fn: |writer, inst| write_immediate_instruction(writer, inst),
    };
}

pub mod cmp {
    use crate::instructions::{
        common::{
            parse_typical_instruction, write_immediate_instruction, write_typical_instruction,
        },
        Description,
    };

    use super::parse_immediate_to_accumulator;

    pub const TO_REGISTER: Description = Description {
        parse_fn: |b| parse_typical_instruction("cmp", b, &TO_REGISTER),
        write_fn: |writer, inst| write_typical_instruction(writer, inst),
    };
    pub const IMMEDIATE_WITH_ACCUMULATOR: Description = Description {
        parse_fn: |bytes| parse_immediate_to_accumulator("cmp", bytes, &IMMEDIATE_WITH_ACCUMULATOR),
        write_fn: |writer, inst| write_immediate_instruction(writer, inst),
    };
}

pub const IMMEDIATE_TO_REGISTER_MEMORY: Description = Description {
    parse_fn: parse_arithmetic_imm_to_register_memory,
    write_fn: write_arithmetic_imm_to_register_memory,
};
