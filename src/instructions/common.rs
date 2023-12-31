use crate::{writer::Writer, Instruction};

use self::instruction_flags::has_word_flag;

use super::opcode::Opcode;

pub mod mode {
    pub const MEMORY_MODE: u8 = 0b00;
    pub const BYTE_DISPLACEMENT: u8 = 0b01;
    pub const WORD_DISPLACEMENT: u8 = 0b10;
    pub const REGISTER_MODE: u8 = 0b11;
}

/// Fetches the mode from the given byte's 2 most significant bits.
#[inline]
pub fn get_mode(byte: u8) -> u8 {
    byte >> 6
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

impl InstRegister {
    pub fn to_str(self) -> &'static str {
        match self {
            InstRegister::Reg(reg) => WORD_REGISTER_STRINGS[reg as usize],
            InstRegister::SegReg(reg) => SEGMENT_REGISTER_STRINGS[reg as usize],
        }
    }
}

impl From<InstRegister> for u8 {
    fn from(val: InstRegister) -> Self {
        match val {
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
        RM::Reg(get_register(byte))
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

pub mod instruction_flags {
    /// Instruction operates on word
    pub const WORD: u8 = 1 << 0;
    /// Sign extend 8-bit immediate data to 16 bits if word flag is set
    pub const SIGN: u8 = 1 << 1;
    /// If set, instruction destination is specified in the register field,
    /// otherwise the source is in the register field
    pub const DIRECTION: u8 = 1 << 2;
    /// If unset, shift/rotate count is one, otherwise it is in the CL register
    pub const SHIFT_ROTATE: u8 = 1 << 3;
    /// If unset, repeat/loop while zero flag is cleared, otherwise repeat/loop
    /// while zero flag is set
    pub const ZERO: u8 = 1 << 4;

    pub const SET: u8 = WORD | SIGN | DIRECTION | SHIFT_ROTATE | ZERO;

    #[inline]
    pub fn has_word_flag(flags: u8) -> bool {
        flags & WORD == WORD
    }

    #[inline]
    pub fn has_sign_flag(flags: u8) -> bool {
        flags & SIGN == SIGN
    }

    #[inline]
    pub fn has_direction_flag(flags: u8) -> bool {
        flags & DIRECTION == DIRECTION
    }

    #[inline]
    pub fn has_shift_rotate_flag(flags: u8) -> bool {
        flags & SHIFT_ROTATE == SHIFT_ROTATE
    }
}

pub fn parse_instruction_flags(byte: u8) -> u8 {
    let mut flags = 0;

    let first_flag = byte & 0b1 == 0b1;
    let second_flag = (byte >> 1) & 0b1 == 0b1;

    if first_flag {
        flags |= instruction_flags::WORD;
        flags |= instruction_flags::ZERO;
    }

    if second_flag {
        flags |= instruction_flags::SIGN;
        flags |= instruction_flags::DIRECTION;
        flags |= instruction_flags::SHIFT_ROTATE;
    }

    flags
}

#[test]
fn test_parse_instruction_flags() {
    let flags = parse_instruction_flags(0b00000001);
    assert_eq!(flags & instruction_flags::WORD, instruction_flags::WORD);
    assert_eq!(flags & instruction_flags::SIGN, 0);
    assert_eq!(flags & instruction_flags::DIRECTION, 0);
    assert_eq!(flags & instruction_flags::SHIFT_ROTATE, 0);
    assert_eq!(flags & instruction_flags::ZERO, instruction_flags::ZERO);

    let flags = parse_instruction_flags(0b00000010);
    assert_eq!(flags & instruction_flags::WORD, 0);
    assert_eq!(flags & instruction_flags::SIGN, instruction_flags::SIGN);
    assert_eq!(
        flags & instruction_flags::DIRECTION,
        instruction_flags::DIRECTION
    );
    assert_eq!(
        flags & instruction_flags::SHIFT_ROTATE,
        instruction_flags::SHIFT_ROTATE
    );
    assert_eq!(flags & instruction_flags::ZERO, 0);
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
        .write_destination(instruction)
        .write_comma_separator()
        .write_source(instruction)
        .end_line();
}

pub fn parse_typical_instruction(inst: &mut Instruction, opcode: Opcode, bytes: &[u8]) {
    let displacement = get_displacement_amount(bytes[1]);

    inst.opcode = opcode;
    inst.length = 2 + displacement;
    inst.flags = parse_instruction_flags(bytes[0]);
    inst.register = get_register(bytes[1] >> 3);
    inst.data_fields = InstructionDataFields::parse(bytes[1]);
    inst.disp = get_disp_value(bytes, displacement, 2);
}

pub fn write_immediate_instruction(writer: &mut Writer, instruction: &Instruction) {
    writer
        .start_instruction(instruction)
        .write_destination(instruction)
        .write_comma_separator();

    if instruction.opcode == Opcode::MOV {
        writer.write_with_w_flag(instruction.data, instruction);
    } else {
        writer.write_signed_data(instruction);
    }

    writer.end_line();
}

#[inline]
pub fn create_single_byte_instruction(
    inst: &mut Instruction,
    opcode: Opcode,
    register: InstRegister,
) {
    inst.opcode = opcode;
    inst.length = 1;
    inst.flags = instruction_flags::SET;
    inst.register = register;
}

#[inline]
pub fn parse_bare_instruction(inst: &mut Instruction, opcode: Opcode) {
    inst.opcode = opcode;
    inst.length = 1;
}

#[inline]
pub fn write_bare_instruction(writer: &mut Writer, instruction: &Instruction) {
    writer.start_instruction(instruction).end_line();
}

pub fn write_memory_or_register_instruction(writer: &mut Writer, inst: &Instruction) {
    writer.start_instruction(inst);

    if let RM::Eff(_) = inst.data_fields.rm {
        if has_word_flag(inst.flags) {
            writer.write_str("word ");
        } else {
            writer.write_str("byte ");
        }
    }

    writer.write_rm(inst).end_line();
}
