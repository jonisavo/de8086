use crate::{writer::Writer, Instruction};

use super::Description;

pub mod mode {
    pub const MEMORY_MODE: u8 = 0b00;
    pub const BYTE_DISPLACEMENT: u8 = 0b01;
    pub const WORD_DISPLACEMENT: u8 = 0b10;
    pub const REGISTER_MODE: u8 = 0b11;
}

/// Fetches the mode from the given byte's 2 most significant bits.
#[inline]
pub fn get_mode(byte: u8) -> u8 {
    return byte >> 6;
}

#[test]
fn test_get_mode() {
    assert_eq!(get_mode(0b00000000), mode::MEMORY_MODE);
    assert_eq!(get_mode(0b01000000), mode::BYTE_DISPLACEMENT);
    assert_eq!(get_mode(0b10000000), mode::WORD_DISPLACEMENT);
    assert_eq!(get_mode(0b11000000), mode::REGISTER_MODE);
}

pub fn get_displacement_amount(byte: u8) -> u8 {
    match get_mode(byte) {
        mode::MEMORY_MODE => {
            if get_rm_effective_address(byte) == EFFECTIVE_DIRECT_ADDRESS {
                2
            } else {
                0
            }
        }
        mode::BYTE_DISPLACEMENT => 1,
        mode::WORD_DISPLACEMENT => 2,
        mode::REGISTER_MODE => 0,
        mode => unreachable!("Invalid mode: {}", mode),
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

pub fn get_disp_value(bytes: &[u8], displacement: u8, offset: usize) -> i16 {
    match displacement {
        1 => bytes[offset] as i8 as i16,
        2 => ((bytes[offset + 1] as i16) << 8) | bytes[offset] as i16,
        _ => 0,
    }
}

pub fn get_data_value(bytes: &[u8], word: bool, offset: usize) -> u16 {
    if word {
        ((bytes[offset + 1] as u16) << 8) | bytes[offset] as u16
    } else {
        bytes[offset] as u16
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

pub mod register {
    pub const AX: u8 = 0b000;
    pub const CX: u8 = 0b001;
    pub const DX: u8 = 0b010;
    pub const BX: u8 = 0b011;
    pub const SP: u8 = 0b100;
    pub const BP: u8 = 0b101;
    pub const SI: u8 = 0b110;
    pub const DI: u8 = 0b111;
}

/// Fetches the register from the given byte's 3 least significant bits.
pub fn get_register(value: u8) -> InstRegister {
    InstRegister::Reg(value & 0b111)
}

#[test]
fn test_get_register() {
    assert_eq!(get_register(0b000), InstRegister::Reg(register::AX));
    assert_eq!(get_register(0b1001), InstRegister::Reg(register::CX));
    assert_eq!(get_register(0b10010), InstRegister::Reg(register::DX));
    assert_eq!(get_register(0b011), InstRegister::Reg(register::BX));
    assert_eq!(get_register(0b100), InstRegister::Reg(register::SP));
    assert_eq!(get_register(0b101), InstRegister::Reg(register::BP));
    assert_eq!(get_register(0b11111110), InstRegister::Reg(register::SI));
    assert_eq!(get_register(0b111), InstRegister::Reg(register::DI));
}

pub mod segment_register {
    pub const ES: u8 = 0b00;
    pub const CS: u8 = 0b01;
    pub const SS: u8 = 0b10;
    pub const DS: u8 = 0b11;
}

/// Fetches the register from the given byte's 2 least significant bits.
pub fn get_segment_register(value: u8) -> InstRegister {
    InstRegister::SegReg(value & 0b11)
}

pub const SEGMENT_REGISTER_STRINGS: [&str; 4] = ["es", "cs", "ss", "ds"];

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InstRegister {
    Reg(u8),
    SegReg(u8),
}

impl Into<u8> for InstRegister {
    fn into(self) -> u8 {
        match self {
            InstRegister::Reg(reg) => reg,
            InstRegister::SegReg(reg) => reg,
        }
    }
}

pub mod effective {
    pub const BX_PLUS_SI: u8 = 0b000;
    pub const BX_PLUS_DI: u8 = 0b001;
    pub const BP_PLUS_SI: u8 = 0b010;
    pub const BP_PLUS_DI: u8 = 0b011;
    pub const SI: u8 = 0b100;
    pub const DI: u8 = 0b101;
    pub const BP_OR_DIRECT_ADDRESS: u8 = 0b110;
    pub const BX: u8 = 0b111;
}

pub const EFFECTIVE_ADDRESS_STRINGS: [&str; 8] = [
    "[bx+si", "[bx+di", "[bp+si", "[bp+di", "[si", "[di", "[bp", "[bx",
];

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RM {
    Reg(InstRegister),
    Eff(u8),
}

const EFFECTIVE_DIRECT_ADDRESS: RM = RM::Eff(effective::BP_OR_DIRECT_ADDRESS);

/// Fetches the R/M value from the given byte's 3 least significant bits.
pub fn get_rm(byte: u8, mode: u8) -> RM {
    if mode == mode::REGISTER_MODE {
        RM::Reg(get_register(byte).into())
    } else {
        get_rm_effective_address(byte)
    }
}

#[test]
fn test_get_rm() {
    assert_eq!(
        get_rm(0b011, mode::REGISTER_MODE),
        RM::Reg(InstRegister::Reg(register::BX))
    );
    assert_eq!(
        get_rm(0b01110, mode::MEMORY_MODE),
        RM::Eff(effective::BP_OR_DIRECT_ADDRESS)
    );
    assert_eq!(
        get_rm(0b11111111, mode::BYTE_DISPLACEMENT),
        RM::Eff(effective::BX)
    );
    assert_eq!(
        get_rm(0b010, mode::WORD_DISPLACEMENT),
        RM::Eff(effective::BP_PLUS_SI)
    );
}

fn get_rm_effective_address(byte: u8) -> RM {
    RM::Eff(byte & 0b111)
}

pub const WORD_REGISTER_STRINGS: [&str; 8] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];

pub const BYTE_REGISTER_STRINGS: [&str; 8] = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];

#[derive(Debug, Copy, Clone)]
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

impl InstructionFields {
    pub const EMPTY: InstructionFields = InstructionFields {
        word: false,
        sign: false,
        direction: false,
        shift_rotate: false,
        zero: false,
    };
    pub const SET: InstructionFields = InstructionFields {
        word: true,
        sign: true,
        direction: true,
        shift_rotate: true,
        zero: true,
    };

    pub const fn parse(byte: u8) -> InstructionFields {
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
}

#[test]
fn test_parse_instruction_fields() {
    let fields = InstructionFields::parse(0b00000001);
    assert_eq!(fields.word, true);
    assert_eq!(fields.sign, false);
    assert_eq!(fields.direction, false);
    assert_eq!(fields.shift_rotate, false);
    assert_eq!(fields.zero, true);

    let fields = InstructionFields::parse(0b00000010);
    assert_eq!(fields.word, false);
    assert_eq!(fields.sign, true);
    assert_eq!(fields.direction, true);
    assert_eq!(fields.shift_rotate, true);
    assert_eq!(fields.zero, false);
}

#[derive(Debug, Copy, Clone)]
pub struct InstructionDataFields {
    pub mode: u8,
    pub rm: RM,
}

impl InstructionDataFields {
    pub const EMPTY: InstructionDataFields = InstructionDataFields {
        mode: mode::REGISTER_MODE,
        rm: RM::Reg(InstRegister::Reg(register::AX)),
    };
    pub const DIRECT_ADDRESS: InstructionDataFields = InstructionDataFields {
        mode: mode::MEMORY_MODE,
        rm: RM::Eff(effective::BP_OR_DIRECT_ADDRESS),
    };

    pub fn parse(byte: u8) -> InstructionDataFields {
        let mode = get_mode(byte);
        let rm = get_rm(byte, mode);

        InstructionDataFields { mode, rm }
    }
}

#[test]
fn test_instruction_data_fields_parse() {
    let fields = InstructionDataFields::parse(0b11000001);
    assert_eq!(fields.mode, mode::REGISTER_MODE);
    assert_eq!(fields.rm, RM::Reg(InstRegister::Reg(register::CX)));

    let fields = InstructionDataFields::parse(0b00000100);
    assert_eq!(fields.mode, mode::MEMORY_MODE);
    assert_eq!(fields.rm, RM::Eff(effective::SI));
}

pub fn write_typical_instruction(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_str(&instruction.destination_string())
        .write_comma_separator()
        .write_str(&instruction.source_string())
        .end_line();
}

pub fn parse_typical_instruction(
    inst: &mut Instruction,
    mnemonic: &'static str,
    bytes: &[u8],
    description: &'static Description,
) {
    let displacement = get_displacement_amount(bytes[1]);

    inst.mnemonic = mnemonic;
    inst.length = 2 + displacement;
    inst.fields = InstructionFields::parse(bytes[0]);
    inst.register = get_register(bytes[1] >> 3);
    inst.data_fields = InstructionDataFields::parse(bytes[1]);
    inst.disp = get_disp_value(&bytes, displacement, 2);
    inst.description = description;
}

pub fn write_immediate_instruction(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_str(&instruction.destination_string())
        .write_comma_separator();

    if instruction.mnemonic == "mov" {
        writer.write_with_w_flag(instruction.data, instruction);
    } else {
        let signed_data = if instruction.fields.word {
            instruction.data as i16
        } else {
            instruction.data as i8 as i16
        };
        writer.write_str(&signed_data.to_string());
    }

    writer.end_line();
}

pub fn create_single_byte_instruction(
    inst: &mut Instruction,
    mnemonic: &'static str,
    description: &'static Description,
    register: InstRegister,
) {
    inst.mnemonic = mnemonic;
    inst.length = 1;
    inst.fields = InstructionFields::SET;
    inst.register = register;
    inst.description = description;
}

pub fn write_bare_instruction(writer: &mut Writer, instruction: &Instruction) {
    writer.start_instruction(instruction).end_line();
}
