use crate::writer::Writer;

use super::{
    common::{register, InstRegister, InstructionDataFields, InstructionFields, RM},
    descriptions::{Description, UNIMPLEMENTED},
    resolve,
};

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub mnemonic: &'static str,
    pub length: u8,
    pub data_fields: InstructionDataFields,
    pub disp: i16,
    pub data: u16,
    pub fields: InstructionFields,
    pub register: InstRegister,
    pub description: &'static Description,
    pub input: [u8; 6],
}

impl Instruction {
    pub const EMPTY: Instruction = Instruction {
        mnemonic: "",
        length: 0,
        data_fields: InstructionDataFields::EMPTY,
        disp: 0,
        data: 0,
        fields: InstructionFields::EMPTY,
        register: InstRegister::Reg(register::AX),
        description: &UNIMPLEMENTED,
        input: [0; 6],
    };

    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == 0 {
            return None;
        }

        let mut instruction = Instruction::EMPTY;

        let description = resolve(bytes);
        description.parse(bytes, &mut instruction);

        if instruction.length == 0 {
            return None;
        }

        for i in 0..instruction.length as usize {
            instruction.input[i] = bytes[i];
        }

        instruction.description = description;

        Some(instruction)
    }

    pub fn write(&self, writer: &mut Writer) {
        (self.description.write_fn)(writer, self);
    }

    pub fn get_source(&self) -> RM {
        if self.fields.direction {
            self.data_fields.rm
        } else {
            RM::Reg(self.register)
        }
    }

    pub fn get_destination(&self) -> RM {
        if self.fields.direction {
            RM::Reg(self.register)
        } else {
            self.data_fields.rm
        }
    }
}
