use crate::{writer::Writer, Instruction};

use super::{
    common::{
        get_data_value, get_disp_value, get_displacement_amount, get_register,
        parse_typical_instruction, register, write_bare_instruction, write_immediate_instruction,
        write_typical_instruction, InstRegister, InstructionDataFields, InstructionFields, RM,
    },
    Description,
};

pub const ARITHMETIC_MNEMONICS: [&str; 8] = ["add", "or", "adc", "sbb", "and", "sub", "xor", "cmp"];

pub fn parse_arithmetic_imm_to_register_memory(bytes: &[u8], inst: &mut Instruction) {
    let fields = InstructionFields::parse(bytes[0]);
    let displacement = get_displacement_amount(bytes[1]);
    let has_u16_immediate = fields.word && !fields.sign;
    let immediate_length = has_u16_immediate as u8 + 1;
    let data = get_data_value(bytes, has_u16_immediate, 2 + displacement as usize);
    let register = get_register(bytes[1] >> 3);
    let register_number = <InstRegister as Into<u8>>::into(register);

    inst.mnemonic = ARITHMETIC_MNEMONICS[register_number as usize];
    inst.length = 2 + displacement + immediate_length;
    inst.fields = fields;
    inst.register = register;
    inst.data_fields = InstructionDataFields::parse(bytes[1]);
    inst.disp = get_disp_value(&bytes, displacement, 2);
    inst.data = data;
    inst.description = &IMMEDIATE_TO_REGISTER_MEMORY;
}

pub fn write_arithmetic_imm_to_register_memory(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_str(&instruction.address_to_string(instruction.data_fields.rm))
        .write_comma_separator()
        .write_with_size(instruction.data, instruction)
        .end_line();
}

pub fn parse_immediate_to_accumulator(
    inst: &mut Instruction,
    mnemonic: &'static str,
    bytes: &[u8],
    description: &'static Description,
) {
    let fields = InstructionFields::parse(bytes[0]);
    let length = fields.word as u8 + 2;
    let data = get_data_value(bytes, fields.word, 1);

    inst.mnemonic = mnemonic;
    inst.length = length;
    inst.fields = fields;
    inst.register = InstRegister::Reg(register::AX);
    inst.data = data;
    inst.description = description;
}

pub const ADD_TO_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "add", bytes, &ADD_TO_REGISTER),
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const ADD_IMMEDIATE_TO_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| {
        parse_immediate_to_accumulator(inst, "add", bytes, &ADD_IMMEDIATE_TO_ACCUMULATOR)
    },
    write_fn: |writer, inst| write_immediate_instruction(writer, inst),
};

pub const ADC_TO_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "adc", bytes, &ADC_TO_REGISTER),
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const ADC_IMMEDIATE_TO_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| {
        parse_immediate_to_accumulator(inst, "adc", bytes, &ADC_IMMEDIATE_TO_ACCUMULATOR)
    },
    write_fn: |writer, inst| write_immediate_instruction(writer, inst),
};

pub const INC_REGISTER_OR_MEMORY: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "inc", bytes, &INC_REGISTER_OR_MEMORY),
    write_fn: |writer, inst| {
        writer.start_instruction(inst);

        if let RM::Eff(_) = inst.data_fields.rm {
            if inst.fields.word {
                writer.write_str("word ");
            } else {
                writer.write_str("byte ");
            }
        }

        writer
            .write_str(&inst.address_to_string(inst.data_fields.rm))
            .end_line();
    },
};
pub const INC_REGISTER: Description = Description {
    parse_fn: |bytes, inst| {
        inst.mnemonic = "inc";
        inst.length = 1;
        inst.register = get_register(bytes[0] & 0b111);
        inst.fields.word = true;
        inst.description = &INC_REGISTER;
    },
    write_fn: |writer, inst| {
        writer
            .start_instruction(inst)
            .write_str(&inst.register.to_str())
            .end_line();
    },
};

pub const SUB_FROM_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "sub", bytes, &SUB_FROM_REGISTER),
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const SUB_IMMEDIATE_FROM_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| {
        parse_immediate_to_accumulator(inst, "sub", bytes, &SUB_IMMEDIATE_FROM_ACCUMULATOR)
    },
    write_fn: |writer, inst| write_immediate_instruction(writer, inst),
};

pub const CMP_WITH_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "cmp", bytes, &CMP_WITH_REGISTER),
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const CMP_IMMEDIATE_WITH_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| {
        parse_immediate_to_accumulator(inst, "cmp", bytes, &CMP_IMMEDIATE_WITH_ACCUMULATOR)
    },
    write_fn: |writer, inst| write_immediate_instruction(writer, inst),
};

pub const IMMEDIATE_TO_REGISTER_MEMORY: Description = Description {
    parse_fn: parse_arithmetic_imm_to_register_memory,
    write_fn: write_arithmetic_imm_to_register_memory,
};

pub const AAA_DAA: Description = Description {
    parse_fn: |bytes, inst| {
        if bytes[0] == 0b00110111 {
            inst.mnemonic = "aaa"
        } else {
            inst.mnemonic = "daa"
        }
        inst.length = 1;
        inst.description = &AAA_DAA;
    },
    write_fn: write_bare_instruction,
};
