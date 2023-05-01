use std::collections::HashMap;

use crate::Instruction;

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

pub struct Writer {
    file_buffer: Vec<u8>,
    next_instruction_byte_index: usize,
    current_instruction_byte_index: usize,
    instruction_buffer: Vec<WrittenInstruction>,
    label_map: HashMap<usize, Label>,
    current_instruction: Option<WrittenInstruction>,
    options: WriterOptions,
}

impl Writer {
    pub fn new(options: WriterOptions) -> Self {
        let mut buffer = Vec::new();
        let mut instruction_buffer = Vec::new();
        let mut label_map = HashMap::new();

        buffer.reserve(32);
        instruction_buffer.reserve(32);
        label_map.reserve(16);

        Self {
            file_buffer: buffer,
            next_instruction_byte_index: 0,
            current_instruction_byte_index: 0,
            instruction_buffer,
            label_map,
            current_instruction: None,
            options,
        }
    }

    fn write_instruction_input(&mut self, instruction: &Instruction) {
        self.write_str("; ");

        assert!(instruction.length <= 6, "Instruction length is too long.");

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

        let written_instruction = WrittenInstruction {
            start_instruction_index: self.current_instruction_byte_index,
            start_file_index: self.file_buffer.len(),
            length: instruction.length,
        };

        self.instruction_buffer.push(written_instruction);

        self.write_str(instruction.mnemonic).write_byte(b' ');

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
        self.write(string.as_bytes())
    }

    pub fn write_comma_separator(&mut self) -> &mut Self {
        self.write(b", ")
    }

    pub fn write_comment(&mut self, comment: &str) -> &mut Self {
        self.file_buffer.reserve(comment.len() + 2);
        self.write(b"; ").write_str(comment).end_line()
    }

    pub fn write_with_w_flag(&mut self, value: u16, instruction: &Instruction) -> &mut Self {
        self.file_buffer.reserve(6);

        if instruction.fields.word {
            self.write(format!("{:#006x}", value).as_bytes())
        } else {
            self.write(format!("{:#004x}", value).as_bytes())
        }
    }

    pub fn write_with_size(&mut self, value: u16, instruction: &Instruction) -> &mut Self {
        self.file_buffer.reserve(5);

        let writer = if instruction.fields.word {
            self.write(b"word ")
        } else {
            self.write(b"byte ")
        };

        writer.write_with_w_flag(value, instruction)
    }

    pub fn write_jump_displacement(&mut self, displacement: i8) -> &mut Self {
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
            .instruction_buffer
            .last()
            .copied()
            .expect("No instruction to go back from");

        let abs_displacement = displacement.abs() as usize;
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

    let mov_instruction = Instruction::parse(&[0b1011_0000, 0b0000_0000]).unwrap();
    let add_instruction = Instruction::parse(&[0b0000_0100, 0b0000_0000]).unwrap();
    let sub_instruction = Instruction::parse(&[0b0010_1100, 0b0000_0000]).unwrap();
    let je_instruction = Instruction::parse(&[0b0111_0100, 0b0000_0000]).unwrap();

    writer
        .start_instruction(&add_instruction)
        .write_str("ax, bx")
        .end_line();
    writer
        .start_instruction(&sub_instruction)
        .write_str("ax, bx")
        .end_line();
    let length = je_instruction.length + sub_instruction.length + add_instruction.length;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(-(length as i8))
        .end_line();
    let length = length + je_instruction.length;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(-(length as i8))
        .end_line();
    writer
        .start_instruction(&add_instruction)
        .write_str("ax, bx")
        .end_line();
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(-(je_instruction.length as i8))
        .end_line();
    let length = je_instruction.length * 2;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(-(length as i8))
        .end_line();
    writer
        .start_instruction(&mov_instruction)
        .write_str("ax, bx")
        .end_line();
    writer
        .start_instruction(&sub_instruction)
        .write_str("ax, bx")
        .end_line();
    let length = je_instruction.length * 3 + sub_instruction.length + mov_instruction.length;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(-(length as i8))
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
je loc_1
"
    );
}

#[test]
fn test_writer_labels_add_after() {
    let mut writer = Writer::new(WriterOptions { verbose: false });

    let add_instruction = Instruction::parse(&[0b0000_0100, 0b0000_0000]).unwrap();
    let sub_instruction = Instruction::parse(&[0b0010_1100, 0b0000_0000]).unwrap();
    let je_instruction = Instruction::parse(&[0b0111_0100, 0b0000_0000]).unwrap();

    let length = je_instruction.length * 2;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(length as i8)
        .end_line();
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(je_instruction.length as i8)
        .end_line();
    let length = add_instruction.length + sub_instruction.length;
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(length as i8)
        .end_line();
    writer
        .start_instruction(&add_instruction)
        .write_str("ax, bx")
        .end_line();
    writer
        .start_instruction(&sub_instruction)
        .write_str("ax, bx")
        .end_line();
    writer
        .start_instruction(&je_instruction)
        .write_jump_displacement(0)
        .end_line();
    writer
        .start_instruction(&add_instruction)
        .write_str("ax, bx")
        .end_line();

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
