use crate::instructions::common::{Instruction, OpCode};
use crate::instructions::mov::write_mov;
use std::io::BufWriter;

impl Instruction {
    pub fn write<T: std::io::Write>(&self, writer: &mut BufWriter<T>) {
        match self.header.op_code {
            OpCode::MOV => write_mov(writer, self),
        }
    }
}
