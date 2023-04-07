#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    MOV = 0b100010,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == OpCode::MOV as u8 => Ok(OpCode::MOV),
            _ => Err(()),
        }
    }
}

pub fn get_mode(instruction: u16) -> Mode {
    let first_byte = instruction as u8 >> 6;
    match first_byte & 0b11 {
        0b00 => {
            if get_rm_effective_address(instruction) == RM::Effective(RMValue::DirectAddress) {
                Mode::WordDisplacement
            } else {
                Mode::MemoryMode
            }
        }
        0b01 => Mode::ByteDisplacement,
        0b10 => Mode::WordDisplacement,
        0b11 => Mode::RegisterMode,
        _ => unreachable!(),
    }
}

pub fn get_rm(instruction: u16, mode: Mode) -> RM {
    if mode == Mode::RegisterMode {
        get_rm_register(instruction)
    } else {
        get_rm_effective_address(instruction)
    }
}

pub fn get_to_register(instruction: u16) -> bool {
    (instruction >> 9) & 0b1 == 0b1
}

pub fn get_word(instruction: u16) -> bool {
    (instruction >> 8) & 0b1 == 0b1
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    MemoryMode = 0b00,
    ByteDisplacement = 0b01,
    WordDisplacement = 0b10,
    RegisterMode = 0b11,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Register {
    AX = 0b000,
    CX = 0b001,
    DX = 0b010,
    BX = 0b011,
    SP = 0b100,
    BP = 0b101,
    SI = 0b110,
    DI = 0b111,
}

pub fn get_register(value: u8) -> Register {
    match value & 0b111 {
        0b000 => Register::AX,
        0b001 => Register::CX,
        0b010 => Register::DX,
        0b011 => Register::BX,
        0b100 => Register::SP,
        0b101 => Register::BP,
        0b110 => Register::SI,
        0b111 => Register::DI,
        _ => unreachable!(),
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RMValue {
    BXPlusSI = 0b000,
    BXPlusDI = 0b001,
    BPPlusSI = 0b010,
    BPPlusDI = 0b011,
    SI = 0b100,
    DI = 0b101,
    DirectAddress = 0b110,
    BX = 0b111,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RM {
    Register(Register),
    Effective(RMValue),
}

pub fn get_rm_effective_address(instruction: u16) -> RM {
    match instruction & 0b111 {
        0b000 => RM::Effective(RMValue::BXPlusSI),
        0b001 => RM::Effective(RMValue::BXPlusDI),
        0b010 => RM::Effective(RMValue::BPPlusSI),
        0b011 => RM::Effective(RMValue::BPPlusDI),
        0b100 => RM::Effective(RMValue::SI),
        0b101 => RM::Effective(RMValue::DI),
        0b110 => RM::Effective(RMValue::DirectAddress),
        0b111 => RM::Effective(RMValue::BX),
        _ => unreachable!(),
    }
}

pub fn get_rm_register(instruction: u16) -> RM {
    RM::Register(get_register(instruction << 3))
}

pub fn get_word_register_string(register: Register) -> &'static str {
    match register {
        Register::AX => "ax",
        Register::CX => "cx",
        Register::DX => "dx",
        Register::BX => "bx",
        Register::SP => "sp",
        Register::BP => "bp",
        Register::SI => "si",
        Register::DI => "di",
    }
}

pub fn get_byte_register_string(register: Register) -> &'static str {
    match register {
        Register::AX => "al",
        Register::CX => "cl",
        Register::DX => "dl",
        Register::BX => "bl",
        Register::SP => "ah",
        Register::BP => "ch",
        Register::SI => "dh",
        Register::DI => "bh",
    }
}

#[derive(Debug)]
pub struct InstructionFields {
    pub to_register: bool,
    pub word: bool,
}

#[derive(Debug)]
pub struct InstructionHeader {
    pub op_code: OpCode,
    pub register: Register,
    pub fields: InstructionFields,
}

#[derive(Debug)]
pub struct InstructionDataFields {
    pub mode: Mode,
    pub rm: RM,
}

#[derive(Debug)]
pub enum InstructionData {
    Fields(InstructionDataFields),
    Data(u16),
}

pub fn parse_instruction_header(data: u8) -> Result<InstructionHeader, ()> {
    let op_code = OpCode::try_from(data >> 2)?;

    Ok(InstructionHeader {
        op_code,
        register: get_register(data),
        fields: InstructionFields {
            to_register: get_to_register(data),
            word: get_word(data),
        },
    })
}

pub fn parse_instruction_data_fields(data: u16) -> InstructionData {
    let mode = get_mode(data);

    InstructionData::Fields(InstructionDataFields {
        mode,
        rm: get_rm(data, mode),
    })
}

#[derive(Debug)]
pub struct Instruction {
    pub header: InstructionHeader,
    pub data: InstructionData,
    pub disp: u16,
    pub additional_data: u16,
}
