use crate::writer::Writer;

use super::{
    common::{
        effective, mode, register, InstRegister, InstructionDataFields, InstructionFields,
        BYTE_REGISTER_STRINGS, EFFECTIVE_ADDRESS_STRINGS, RM, SEGMENT_REGISTER_STRINGS,
        WORD_REGISTER_STRINGS,
    },
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

    fn get_source(&self) -> RM {
        if self.fields.direction {
            self.data_fields.rm
        } else {
            RM::Reg(self.register)
        }
    }

    fn get_destination(&self) -> RM {
        if self.fields.direction {
            RM::Reg(self.register)
        } else {
            self.data_fields.rm
        }
    }

    fn register_to_str(&self, register: InstRegister) -> &str {
        match register {
            InstRegister::Reg(reg) => {
                if self.fields.word {
                    &WORD_REGISTER_STRINGS[reg as usize]
                } else {
                    &BYTE_REGISTER_STRINGS[reg as usize]
                }
            }
            InstRegister::SegReg(reg) => &SEGMENT_REGISTER_STRINGS[reg as usize],
        }
    }

    fn effective_to_string(&self, effective: u8, mode: u8) -> String {
        if effective == effective::BP_OR_DIRECT_ADDRESS && mode == mode::MEMORY_MODE {
            format!("[{}", self.disp)
        } else {
            EFFECTIVE_ADDRESS_STRINGS[effective as usize].to_string()
        }
    }

    pub fn address_to_string(&self, address: RM) -> String {
        let mut string = String::new();

        match address {
            RM::Reg(reg) => {
                string.push_str(self.register_to_str(reg));
            }
            RM::Eff(eff) => {
                let mode = self.data_fields.mode;
                string.push_str(&self.effective_to_string(eff, mode));
                let is_direct_address =
                    eff == effective::BP_OR_DIRECT_ADDRESS && mode == mode::MEMORY_MODE;

                if !is_direct_address && self.disp != 0 {
                    string.push_str(&format!("{:+}", self.disp));
                }

                string.push_str("]");
            }
        }

        string
    }

    pub fn destination_string(&self) -> String {
        self.address_to_string(self.get_destination())
    }

    pub fn source_string(&self) -> String {
        self.address_to_string(self.get_source())
    }
}
