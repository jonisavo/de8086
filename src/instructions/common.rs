#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    MemoryMode = 0b00,
    ByteDisplacement = 0b01,
    WordDisplacement = 0b10,
    RegisterMode = 0b11,
}

/// Fetches the mode from the given byte's 2 most significant bits.
pub fn get_mode(byte: u8) -> Mode {
    match byte >> 6 {
        0b00 => Mode::MemoryMode,
        0b01 => Mode::ByteDisplacement,
        0b10 => Mode::WordDisplacement,
        0b11 => Mode::RegisterMode,
        _ => unreachable!(),
    }
}

#[test]
fn test_get_mode() {
    assert_eq!(get_mode(0b00000000), Mode::MemoryMode);
    assert_eq!(get_mode(0b01000000), Mode::ByteDisplacement);
    assert_eq!(get_mode(0b10000000), Mode::WordDisplacement);
    assert_eq!(get_mode(0b11000000), Mode::RegisterMode);
}

pub fn get_displacement_amount(byte: u8) -> u8 {
    match get_mode(byte) {
        Mode::MemoryMode => {
            if get_rm_effective_address(byte) == EFFECTIVE_DIRECT_ADDRESS {
                2
            } else {
                0
            }
        }
        Mode::ByteDisplacement => 1,
        Mode::WordDisplacement => 2,
        Mode::RegisterMode => 0,
    }
}

#[test]
fn test_get_displacement_amount() {
    assert_eq!(get_displacement_amount(0b00000000), 0);
    assert_eq!(get_displacement_amount(0b00000110), 2);
    assert_eq!(get_displacement_amount(0b01000000), 1);
    assert_eq!(get_displacement_amount(0b10000000), 2);
    assert_eq!(get_displacement_amount(0b11000000), 0);
}

pub fn get_disp_value(bytes: &[u8], displacement: u8, offset: usize) -> u16 {
    match displacement {
        1 => bytes[offset] as u16,
        2 => ((bytes[offset + 1] as u16) << 8) | bytes[offset] as u16,
        _ => 0,
    }
}

#[test]
fn test_get_disp_value() {
    let bytes = [0x01, 0x02, 0x03];

    assert_eq!(get_disp_value(&bytes, 0, 0), 0x00);
    assert_eq!(get_disp_value(&bytes, 1, 0), 0x01);
    assert_eq!(get_disp_value(&bytes, 2, 0), 0x0201);
    assert_eq!(get_disp_value(&bytes, 0, 0), 0x00);
    assert_eq!(get_disp_value(&bytes, 2, 1), 0x0302);
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

/// Fetches the register from the given byte's 3 least significant bits.
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

#[test]
fn test_get_register() {
    assert_eq!(get_register(0b000), Register::AX);
    assert_eq!(get_register(0b1001), Register::CX);
    assert_eq!(get_register(0b10010), Register::DX);
    assert_eq!(get_register(0b011), Register::BX);
    assert_eq!(get_register(0b100), Register::SP);
    assert_eq!(get_register(0b101), Register::BP);
    assert_eq!(get_register(0b11111110), Register::SI);
    assert_eq!(get_register(0b111), Register::DI);
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Effective {
    BXPlusSI = 0b000,
    BXPlusDI = 0b001,
    BPPlusSI = 0b010,
    BPPlusDI = 0b011,
    SI = 0b100,
    DI = 0b101,
    DirectAddress = 0b110,
    BX = 0b111,
}

pub const EFFECTIVE_ADDRESS_STRINGS: [&str; 8] = [
    "[bx+si", "[bx+di", "[bp+si", "[bp+di", "[si", "[di", "[bp", "[bx",
];

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RM {
    Reg(Register),
    Eff(Effective),
}

const EFFECTIVE_DIRECT_ADDRESS: RM = RM::Eff(Effective::DirectAddress);

/// Fetches the R/M value from the given byte's 3 least significant bits.
pub fn get_rm(byte: u8, mode: Mode) -> RM {
    if mode == Mode::RegisterMode {
        get_rm_register(byte)
    } else {
        get_rm_effective_address(byte)
    }
}

#[test]
fn test_get_rm() {
    assert_eq!(get_rm(0b011, Mode::RegisterMode), RM::Reg(Register::BX));
    assert_eq!(
        get_rm(0b01110, Mode::MemoryMode),
        RM::Eff(Effective::DirectAddress)
    );
    assert_eq!(
        get_rm(0b11111111, Mode::ByteDisplacement),
        RM::Eff(Effective::BX)
    );
    assert_eq!(
        get_rm(0b010, Mode::WordDisplacement),
        RM::Eff(Effective::BPPlusSI)
    );
}

fn get_rm_effective_address(byte: u8) -> RM {
    match byte & 0b111 {
        0b000 => RM::Eff(Effective::BXPlusSI),
        0b001 => RM::Eff(Effective::BXPlusDI),
        0b010 => RM::Eff(Effective::BPPlusSI),
        0b011 => RM::Eff(Effective::BPPlusDI),
        0b100 => RM::Eff(Effective::SI),
        0b101 => RM::Eff(Effective::DI),
        0b110 => RM::Eff(Effective::DirectAddress),
        0b111 => RM::Eff(Effective::BX),
        _ => unreachable!(),
    }
}

fn get_rm_register(instruction: u8) -> RM {
    RM::Reg(get_register(instruction))
}

pub const WORD_REGISTER_STRINGS: [&str; 8] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];

pub const BYTE_REGISTER_STRINGS: [&str; 8] = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];

#[derive(Debug)]
pub struct InstructionFields {
    // Instruction operates on word
    pub word: bool,
    /// Sign extend 8-bit immediate data to 16 bits if word flag is set
    pub sign: bool,
    /// If true, instruction destination is specified in the register field,
    /// otherwise the source is in the register field
    pub direction: bool,
    /// If false, shift/rotate count is one, otherwise it is in the CL register
    pub shift_rotate: bool,
    /// If false, repeat/loop while zero flag is cleared, otherwise repeat/loop
    /// while zero flag is set
    pub zero: bool,
}

#[derive(Debug)]
pub struct InstructionDataFields {
    pub mode: Mode,
    pub rm: RM,
}

#[derive(Debug)]
pub enum InstructionData {
    Fields(InstructionDataFields),
    Data(u8),
}

impl InstructionData {
    pub fn parse_fields(byte: u8) -> InstructionData {
        let mode = get_mode(byte);
        let rm = get_rm(byte, mode);

        InstructionData::Fields(InstructionDataFields { mode, rm })
    }
}

#[test]
fn test_instruction_data_fields_parse() {
    let fields = match InstructionData::parse_fields(0b11000001) {
        InstructionData::Fields(fields) => fields,
        _ => unreachable!(),
    };
    assert_eq!(fields.mode, Mode::RegisterMode);
    assert_eq!(fields.rm, RM::Reg(Register::CX));

    let fields = match InstructionData::parse_fields(0b00000100) {
        InstructionData::Fields(fields) => fields,
        _ => unreachable!(),
    };
    assert_eq!(fields.mode, Mode::MemoryMode);
    assert_eq!(fields.rm, RM::Eff(Effective::SI));
}

pub fn parse_instruction_fields(byte: u8) -> InstructionFields {
    let first_flag = byte & 0b1 == 0b1;
    let second_flag = (byte >> 1) & 0b1 == 0b1;

    InstructionFields {
        word: first_flag,
        sign: second_flag,
        direction: second_flag,
        shift_rotate: second_flag,
        zero: first_flag,
    }
}

#[test]
fn test_parse_instruction_fields() {
    let fields = parse_instruction_fields(0b00000001);
    assert_eq!(fields.word, true);
    assert_eq!(fields.sign, false);
    assert_eq!(fields.direction, false);
    assert_eq!(fields.shift_rotate, false);
    assert_eq!(fields.zero, true);

    let fields = parse_instruction_fields(0b00000010);
    assert_eq!(fields.word, false);
    assert_eq!(fields.sign, true);
    assert_eq!(fields.direction, true);
    assert_eq!(fields.shift_rotate, true);
    assert_eq!(fields.zero, false);
}
