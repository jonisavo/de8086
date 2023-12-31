use std::collections::HashMap;
use std::io::Write;

use crate::{
    instructions::common::{
        effective, instruction_flags::has_word_flag, mode, InstRegister, BYTE_REGISTER_STRINGS,
        EFFECTIVE_ADDRESS_STRINGS, RM, SEGMENT_REGISTER_STRINGS, WORD_REGISTER_STRINGS,
    },
    Instruction,
};

#[derive(Debug, Copy, Clone)]
struct WrittenInstruction {
    start_instruction_index: usize,
    start_file_index: usize,
    length: u8,
}

#[derive(Debug, Copy, Clone)]
struct Label {
    number: usize,
    pub inserted: bool,
}

#[derive(Debug, Clone)]
pub struct WriterOptions {
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub struct WriterContext {
    repeat: u8,
    lock: bool,
    segment: u8,
}

pub struct Writer {
    file_buffer: Vec<u8>,
    next_instruction_byte_index: usize,
    current_instruction_byte_index: usize,
    instruction_buffer: Vec<WrittenInstruction>,
    label_map: HashMap<usize, Label>,
    current_instruction: Option<WrittenInstruction>,
    options: WriterOptions,
    context: WriterContext,
}

impl Writer {
    pub fn new(options: WriterOptions) -> Self {
        let buffer = Vec::new();
        let instruction_buffer = Vec::new();
        let mut label_map = HashMap::new();

        label_map.reserve(16);

        Self {
            file_buffer: buffer,
            next_instruction_byte_index: 0,
            current_instruction_byte_index: 0,
            instruction_buffer,
            label_map,
            current_instruction: None,
            options,
            context: WriterContext {
                repeat: 0,
                lock: false,
                // TODO(joni): Change default values to 0 across the board
                segment: 0xff,
            },
        }
    }

    fn push_prefix_instruction(&mut self) {
        self.instruction_buffer.push(WrittenInstruction {
            start_instruction_index: self.current_instruction_byte_index,
            start_file_index: self.file_buffer.len(),
            length: 1,
        });
        self.current_instruction_byte_index += 1;
        self.next_instruction_byte_index += 1;
    }

    pub fn set_repeat_prefix(&mut self, byte: u8) {
        self.push_prefix_instruction();
        self.context.repeat = byte;
    }

    pub fn set_lock_prefix(&mut self) {
        self.push_prefix_instruction();
        self.context.lock = true;
    }

    pub fn set_segment_prefix(&mut self, segment: u8) {
        self.push_prefix_instruction();
        self.context.segment = segment;
    }

    fn write_instruction_input(&mut self, instruction: &Instruction) {
        self.write_str("; ");

        assert!(instruction.length <= 6, "Instruction length is too long.");

        if self.context.segment != 0xff {
            let segment_mask = self.context.segment << 3;
            self.write_str(&format!("{:08b} ", 0b00100110 | segment_mask));
        }

        if self.context.lock {
            self.write_str(&format!("{:08b} ", 0b11110000));
        }

        if self.context.repeat != 0 {
            self.write_str(&format!("{:08b} ", self.context.repeat));
        }

        for i in 0..instruction.length {
            self.write_str(&format!("{:08b} ", instruction.input[i as usize]));
        }

        self.end_line();
    }

    pub fn start_instruction(&mut self, instruction: &Instruction) -> &mut Self {
        let mut label_str = None;

        if let Some(label) = self.label_map.get_mut(&self.current_instruction_byte_index) {
            if !label.inserted {
                label_str = Some(format!("loc_{}:\n", label.number));
                label.inserted = true;
            }
        }

        if let Some(label_str) = label_str {
            self.write_str(&label_str);
        }

        if self.options.verbose {
            self.write_instruction_input(instruction);
        }

        if self.context.lock {
            self.write_str("lock ");
            self.context.lock = false;
        }

        if self.context.repeat != 0 {
            self.write_str("rep ");
            self.context.repeat = 0;
        }

        let written_instruction = WrittenInstruction {
            start_instruction_index: self.current_instruction_byte_index,
            start_file_index: self.file_buffer.len(),
            length: instruction.length,
        };

        self.instruction_buffer.push(written_instruction);

        self.write_str(instruction.opcode.get_mnemonic())
            .write_byte(b' ');

        self.current_instruction = Some(written_instruction);

        self.next_instruction_byte_index += instruction.length as usize;

        self
    }

    pub fn write_byte(&mut self, byte: u8) -> &mut Self {
        self.file_buffer.push(byte);

        self
    }

    pub fn write(&mut self, bytes: &[u8]) -> &mut Self {
        self.file_buffer.extend_from_slice(bytes);

        self
    }

    pub fn write_str(&mut self, string: &str) -> &mut Self {
        write!(self.file_buffer, "{}", string).unwrap();
        self
    }

    pub fn write_comma_separator(&mut self) -> &mut Self {
        write!(self.file_buffer, ", ").unwrap();
        self
    }

    pub fn write_comment(&mut self, comment: &str) -> &mut Self {
        writeln!(self.file_buffer, "; {}", comment).unwrap();
        self
    }

    pub fn write_with_w_flag(&mut self, value: u16, instruction: &Instruction) -> &mut Self {
        self.file_buffer.reserve(6);

        if has_word_flag(instruction.flags) {
            self.write(format!("{:#006x}", value).as_bytes())
        } else {
            self.write(format!("{:#004x}", value).as_bytes())
        }
    }

    pub fn write_size(&mut self, instruction: &Instruction) -> &mut Self {
        self.file_buffer.reserve(5);

        if has_word_flag(instruction.flags) {
            self.write(b"word ")
        } else {
            self.write(b"byte ")
        }
    }

    pub fn write_with_size(&mut self, value: u16, instruction: &Instruction) -> &mut Self {
        self.write_size(instruction)
            .write_with_w_flag(value, instruction)
    }

    pub fn write_signed_data(&mut self, instruction: &Instruction) -> &mut Self {
        let signed_data = if has_word_flag(instruction.flags) {
            instruction.data as i16
        } else {
            instruction.data as i8 as i16
        };
        self.write_str(&signed_data.to_string())
    }

    fn register_to_str(&self, instruction: &Instruction, register: InstRegister) -> &str {
        match register {
            InstRegister::Reg(reg) => {
                if has_word_flag(instruction.flags) {
                    WORD_REGISTER_STRINGS[reg as usize]
                } else {
                    BYTE_REGISTER_STRINGS[reg as usize]
                }
            }
            InstRegister::SegReg(reg) => SEGMENT_REGISTER_STRINGS[reg as usize],
        }
    }

    fn effective_to_string(&self, instruction: &Instruction, effective: u8) -> String {
        let has_memory_mode = instruction.data_fields.mode == mode::MEMORY_MODE;
        if effective == effective::BP_OR_DIRECT_ADDRESS && has_memory_mode {
            format!("[{}", instruction.disp)
        } else {
            EFFECTIVE_ADDRESS_STRINGS[effective as usize].to_string()
        }
    }

    pub fn address_to_string(&mut self, instruction: &Instruction, rm: RM) -> String {
        let mut string = String::new();

        match rm {
            RM::Reg(reg) => {
                string.push_str(self.register_to_str(instruction, reg));
            }
            RM::Eff(eff) => {
                if self.context.segment != 0xff {
                    string.push_str(SEGMENT_REGISTER_STRINGS[self.context.segment as usize]);
                    string.push(':');
                    self.context.segment = 0xff;
                }
                string.push_str(&self.effective_to_string(instruction, eff));
                let mode = instruction.data_fields.mode;
                let is_direct_address =
                    eff == effective::BP_OR_DIRECT_ADDRESS && mode == mode::MEMORY_MODE;

                if !is_direct_address && instruction.disp != 0 {
                    string.push_str(&format!("{:+}", instruction.disp));
                }

                string.push(']');
            }
        }

        string
    }

    pub fn write_source(&mut self, instruction: &Instruction) -> &mut Self {
        let str = self.address_to_string(instruction, instruction.get_source());
        self.write_str(&str)
    }

    pub fn write_destination(&mut self, instruction: &Instruction) -> &mut Self {
        let str = self.address_to_string(instruction, instruction.get_destination());
        self.write_str(&str)
    }

    pub fn write_rm(&mut self, instruction: &Instruction) -> &mut Self {
        let str = self.address_to_string(instruction, instruction.data_fields.rm);
        self.write_str(&str)
    }

    pub fn write_jump_displacement(&mut self, displacement: i16) -> &mut Self {
        let next_inst_byte = self.next_instruction_byte_index;
        let target_index = (next_inst_byte as isize) + (displacement as isize);

        assert!(target_index >= 0);

        let target_index = target_index as usize;
        let new_label_number = self.label_map.len();

        let label = *self.label_map.entry(target_index).or_insert_with(|| Label {
            number: new_label_number,
            inserted: false,
        });

        let label_str = format!("loc_{}", label.number);

        self.write_str(&label_str);

        if displacement >= 0 || label.inserted {
            return self;
        }

        // Insert label to a previous instruction

        let mut current_instruction = self
            .current_instruction
            .expect("No instruction to go back from");

        let abs_displacement = displacement.unsigned_abs() as usize;
        let mut target_instruction = &mut current_instruction;

        for inst in self.instruction_buffer.iter_mut().rev() {
            let byte_difference = next_inst_byte - inst.start_instruction_index;
            if byte_difference == abs_displacement {
                target_instruction = inst;
                break;
            }
        }

        let insert_index = target_instruction.start_file_index;
        let str = format!("{}:\n", label_str);
        let inserted_bytes = str.as_bytes();

        self.file_buffer
            .splice(insert_index..insert_index, inserted_bytes.iter().copied());

        target_instruction.start_file_index += inserted_bytes.len();

        self.label_map.insert(
            target_index,
            Label {
                inserted: true,
                ..label
            },
        );

        self
    }

    pub fn end_line(&mut self) -> &mut Self {
        if let Some(current_instruction) = self.current_instruction.take() {
            self.current_instruction_byte_index += current_instruction.length as usize;
        }

        self.file_buffer.push(b'\n');

        self
    }

    pub fn as_slice(&self) -> &[u8] {
        self.file_buffer.as_slice()
    }
}

#[test]
fn test_writer_labels_add_before() {
    let mut writer = Writer::new(WriterOptions { verbose: false });

    let mov_instruction = Instruction::parse(&[0b1000_1001, 0b1101_1000]).unwrap();
    let add_instruction = Instruction::parse(&[0b0000_0001, 0b1101_1000]).unwrap();
    let sub_instruction = Instruction::parse(&[0b0010_1001, 0b1101_1000]).unwrap();
    let je_instruction = Instruction::parse(&[0b0111_0100, 0b0000_0000]).unwrap();
    let rep_instruction = Instruction::parse(&[0b1111_0011]).unwrap();
    let cmps_instruction = Instruction::parse(&[0b1010_0110]).unwrap();

    add_instruction.write(&mut writer);
    sub_instruction.write(&mut writer);

    let length = je_instruction.length + sub_instruction.length + add_instruction.length;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(-(length as i8 as i16))
        .end_line();

    let length = length + je_instruction.length;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(-(length as i8 as i16))
        .end_line();

    add_instruction.write(&mut writer);

    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(-(je_instruction.length as i8 as i16))
        .end_line();

    let length = je_instruction.length * 2;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(-(length as i8 as i16))
        .end_line();

    mov_instruction.write(&mut writer);
    sub_instruction.write(&mut writer);
    rep_instruction.write(&mut writer);
    cmps_instruction.write(&mut writer);

    let length = je_instruction.length * 3 + sub_instruction.length + mov_instruction.length + 2;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(-(length as i8 as i16))
        .end_line();

    let slice = writer.as_slice();

    assert_eq!(
        std::str::from_utf8(slice).unwrap(),
        "loc_0:
add ax, bx
sub ax, bx
je loc_0
je loc_0
add ax, bx
loc_1:
je loc_1
je loc_1
mov ax, bx
sub ax, bx
rep cmpsb 
je loc_1
"
    );
}

#[test]
fn test_writer_labels_add_after() {
    let mut writer = Writer::new(WriterOptions { verbose: false });

    let add_instruction = Instruction::parse(&[0b0000_0001, 0b1101_1000]).unwrap();
    let sub_instruction = Instruction::parse(&[0b0010_1001, 0b1101_1000]).unwrap();
    let je_instruction = Instruction::parse(&[0b0111_0100, 0b0000_0000]).unwrap();

    let length = je_instruction.length * 2;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(length as i8 as i16)
        .end_line();
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(je_instruction.length as i8 as i16)
        .end_line();
    let length = add_instruction.length + sub_instruction.length;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(length as i8 as i16)
        .end_line();

    add_instruction.write(&mut writer);
    sub_instruction.write(&mut writer);

    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(0)
        .end_line();

    add_instruction.write(&mut writer);

    let slice = writer.as_slice();

    assert_eq!(
        std::str::from_utf8(slice).unwrap(),
        "je loc_0
je loc_0
je loc_1
loc_0:
add ax, bx
sub ax, bx
loc_1:
je loc_2
loc_2:
add ax, bx
"
    );
}
