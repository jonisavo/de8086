use crate::writer::Writer;

use super::{
    common::{
        Effective, InstructionData, InstructionFields, Mode, Register, BYTE_REGISTER_STRINGS,
        EFFECTIVE_ADDRESS_STRINGS, RM, WORD_REGISTER_STRINGS,
    },
    description::Description,
};

#[derive(Debug)]
pub struct Instruction {
    pub length: u8,
    pub data: InstructionData,
    pub disp: u16,
    pub additional_data: u16,
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
            match &self.data {
                InstructionData::Fields(fields) => fields.rm,
                InstructionData::Data(_) => {
                    unreachable!("Can not get source string from data")
                }
            }
        } else {
            RM::Reg(self.register)
        }
    }

    fn get_destination(&self) -> RM {
        if self.fields.direction {
            RM::Reg(self.register)
        } else {
            match &self.data {
                InstructionData::Fields(fields) => fields.rm,
                InstructionData::Data(_) => {
                    unreachable!("Can not get destination string from data")
                }
            }
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
            format!("[{:x}h", self.disp)
        } else {
            EFFECTIVE_ADDRESS_STRINGS[eff as usize].to_string()
        }
    }

    fn address_to_string(&self, address: RM) -> String {
        let mut string = String::new();
        let mut mode = Mode::RegisterMode;

        if let InstructionData::Fields(fields) = &self.data {
            mode = fields.mode;
        }

        match address {
            RM::Reg(reg) => {
                string.push_str(self.register_to_str(reg));
            }
            RM::Eff(eff) => {
                string.push_str(&self.effective_to_string(eff, mode));
                let is_direct_address = eff == Effective::DirectAddress && mode == Mode::MemoryMode;

                if !is_direct_address && self.disp != 0 {
                    string.push_str(&format!("+{:x}h", self.disp));
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
