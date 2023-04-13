use crate::writer::Writer;

use super::{
    common::{
        Effective, InstructionFields, Mode, Register, BYTE_REGISTER_STRINGS,
        EFFECTIVE_ADDRESS_STRINGS, RM, WORD_REGISTER_STRINGS, InstructionDataFields,
    },
    description::Description,
};

#[derive(Debug)]
pub struct Instruction {
    pub length: u8,
    pub data_fields: InstructionDataFields,
    pub disp: i16,
    pub data: u16,
    pub fields: InstructionFields,
    pub register: Register,
    pub description: &'static Description,
}

impl Instruction {
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

    fn register_to_str(&self, register: Register) -> &str {
        if self.fields.word {
            &WORD_REGISTER_STRINGS[register as usize]
        } else {
            &BYTE_REGISTER_STRINGS[register as usize]
        }
    }

    fn effective_to_string(&self, eff: Effective, mode: Mode) -> String {
        if eff == Effective::DirectAddress && mode == Mode::MemoryMode {
            format!("[{}", self.disp)
        } else {
            EFFECTIVE_ADDRESS_STRINGS[eff as usize].to_string()
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
                let is_direct_address = eff == Effective::DirectAddress && mode == Mode::MemoryMode;

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
