use crate::writer::Writer;

use super::{
    common::{
        instruction_flags::has_direction_flag, register, InstRegister, InstructionDataFields, RM,
    },
    descriptions::{Description, UNIMPLEMENTED},
    opcode::Opcode,
    resolve,
};

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub length: u8,
    pub data_fields: InstructionDataFields,
    pub disp: i16,
    pub data: u16,
    pub flags: u8,
    pub register: InstRegister,
    pub description: &'static Description,
    pub input: [u8; 6],
}

impl Instruction {
    pub const EMPTY: Instruction = Instruction {
        opcode: Opcode::UNKNOWN,
        length: 0,
        data_fields: InstructionDataFields::EMPTY,
        disp: 0,
        data: 0,
        flags: 0,
        register: InstRegister::Reg(register::AX),
        description: &UNIMPLEMENTED,
        input: [0; 6],
    };

    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }

        let mut instruction = Instruction::EMPTY;

        let description = resolve(bytes);
        description.parse(bytes, &mut instruction);

        if instruction.length == 0 {
            return None;
        }

        let length = instruction.length as usize;

        instruction.input[..length].copy_from_slice(&bytes[..length]);

        instruction.description = description;

        Some(instruction)
    }

    pub fn write(&self, writer: &mut Writer) {
        (self.description.write_fn)(writer, self);
    }

    pub fn get_source(&self) -> RM {
        if has_direction_flag(self.flags) {
            self.data_fields.rm
        } else {
            RM::Reg(self.register)
        }
    }

    pub fn get_destination(&self) -> RM {
        if has_direction_flag(self.flags) {
            RM::Reg(self.register)
        } else {
            self.data_fields.rm
        }
    }
}
