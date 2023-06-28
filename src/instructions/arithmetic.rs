use crate::{writer::Writer, Instruction};

use super::{
    common::{
        get_data_value, get_disp_value, get_displacement_amount, get_register,
        parse_bare_instruction, parse_typical_instruction, register, write_bare_instruction,
        write_immediate_instruction, write_memory_or_register_instruction,
        write_typical_instruction, InstRegister, InstructionDataFields, InstructionFields,
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
) {
    let fields = InstructionFields::parse(bytes[0]);
    let length = fields.word as u8 + 2;
    let data = get_data_value(bytes, fields.word, 1);

    inst.mnemonic = mnemonic;
    inst.length = length;
    inst.fields = fields;
    inst.register = InstRegister::Reg(register::AX);
    inst.data = data;
}

fn write_only_register_instruction(writer: &mut Writer, inst: &Instruction) {
    writer
        .start_instruction(inst)
        .write_str(&inst.register.to_str())
        .end_line();
}

pub const ADD_TO_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "add", bytes),
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const ADD_IMMEDIATE_TO_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, "add", bytes),
    write_fn: |writer, inst| write_immediate_instruction(writer, inst),
};

pub const ADC_TO_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "adc", bytes),
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const ADC_IMMEDIATE_TO_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, "adc", bytes),
    write_fn: |writer, inst| write_immediate_instruction(writer, inst),
};

pub const INC_REGISTER_OR_MEMORY: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "inc", bytes),
    write_fn: write_memory_or_register_instruction,
};
pub const INC_REGISTER: Description = Description {
    parse_fn: |bytes, inst| {
        inst.mnemonic = "inc";
        inst.length = 1;
        inst.register = get_register(bytes[0] & 0b111);
        inst.fields.word = true;
    },
    write_fn: write_only_register_instruction,
};

pub const SUB_FROM_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "sub", bytes),
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const SUB_IMMEDIATE_FROM_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, "sub", bytes),
    write_fn: |writer, inst| write_immediate_instruction(writer, inst),
};

pub const SBB_FROM_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "sbb", bytes),
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const SBB_IMMEDIATE_FROM_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, "sbb", bytes),
    write_fn: |writer, inst| write_immediate_instruction(writer, inst),
};

pub const DEC_REGISTER_OR_MEMORY: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "dec", bytes),
    write_fn: write_memory_or_register_instruction,
};
pub const DEC_REGISTER: Description = Description {
    parse_fn: |bytes, inst| {
        inst.mnemonic = "dec";
        inst.length = 1;
        inst.register = get_register(bytes[0] & 0b111);
        inst.fields.word = true;
    },
    write_fn: write_only_register_instruction,
};

pub const NEG: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "neg", bytes),
    write_fn: write_memory_or_register_instruction,
};

pub const CMP_WITH_REGISTER: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "cmp", bytes),
    write_fn: |writer, inst| write_typical_instruction(writer, inst),
};
pub const CMP_IMMEDIATE_WITH_ACCUMULATOR: Description = Description {
    parse_fn: |bytes, inst| parse_immediate_to_accumulator(inst, "cmp", bytes),
    write_fn: |writer, inst| write_immediate_instruction(writer, inst),
};

pub const IMMEDIATE_TO_REGISTER_MEMORY: Description = Description {
    parse_fn: parse_arithmetic_imm_to_register_memory,
    write_fn: write_arithmetic_imm_to_register_memory,
};

pub const AAA: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, "aaa"),
    write_fn: write_bare_instruction,
};
pub const DAA: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, "daa"),
    write_fn: write_bare_instruction,
};

pub const AAS: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, "aas"),
    write_fn: write_bare_instruction,
};
pub const DAS: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, "das"),
    write_fn: write_bare_instruction,
};

pub const MUL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "mul", bytes),
    write_fn: write_memory_or_register_instruction,
};
pub const IMUL: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "imul", bytes),
    write_fn: write_memory_or_register_instruction,
};

pub const AAM: Description = Description {
    parse_fn: |_, inst| {
        parse_bare_instruction(inst, "aam");
        inst.length = 2;
    },
    write_fn: write_bare_instruction,
};

pub const DIV: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "div", bytes),
    write_fn: write_memory_or_register_instruction,
};
pub const IDIV: Description = Description {
    parse_fn: |bytes, inst| parse_typical_instruction(inst, "idiv", bytes),
    write_fn: write_memory_or_register_instruction,
};

pub const AAD: Description = Description {
    parse_fn: |_, inst| {
        parse_bare_instruction(inst, "aad");
        inst.length = 2;
    },
    write_fn: write_bare_instruction,
};

pub const CBW: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, "cbw"),
    write_fn: write_bare_instruction,
};

pub const CWD: Description = Description {
    parse_fn: |_, inst| parse_bare_instruction(inst, "cwd"),
    write_fn: write_bare_instruction,
};
