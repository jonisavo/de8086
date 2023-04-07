use crate::instructions::common::{
    get_byte_register_string, get_word_register_string, parse_instruction_header, Instruction,
    OpCode, RM,
};
use crate::instructions::mov::parse_mov;

use super::common::InstructionData;

pub fn parse(instruction: u16) -> Result<Instruction, ()> {
    let header = parse_instruction_header(instruction)?;

    match header.op_code {
        OpCode::MOV => Ok(parse_mov(header, instruction)),
    }
}

impl Instruction {
    pub fn destination_string(&self) -> &'static str {
        if self.header.fields.word {
            match &self.data {
                InstructionData::Fields(fields) => match fields.rm {
                    RM::Register(reg) => get_word_register_string(reg),
                    RM::Effective(_) => unreachable!("Effective address not implemented"),
                },
                InstructionData::Data(_) => unreachable!("Data not implemented"),
            }
        } else {
            match &self.data {
                InstructionData::Fields(fields) => match fields.rm {
                    RM::Register(reg) => get_byte_register_string(reg),
                    RM::Effective(_) => unreachable!("Effective address not implemented"),
                },
                InstructionData::Data(_) => unreachable!("Data not implemented"),
            }
        }
    }

    pub fn source_string(&self) -> &'static str {
        if self.header.fields.word {
            get_word_register_string(self.header.register)
        } else {
            get_byte_register_string(self.header.register)
        }
    }
}
