use crate::instructions::common::{parse_instruction_header, InstructionHeader};

pub struct Parser {
    pub header: InstructionHeader,
    pub in_header: bool,
}

impl Parser {
    pub fn new(header: u8) -> Self {
        Self {
            header: parse_instruction_header(header),
            in_header: true,
        }
    }
}
