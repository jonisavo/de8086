pub mod instructions;
pub use crate::instructions::instruction::Instruction;
pub mod parser;
pub mod writer;

use std::io::{stdout, Read, Write};
use writer::{Writer, WriterOptions};

fn read_file(filename: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(filename)?;

    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn run(
    file_name: &str,
    bytes: &[u8],
    writer_options: WriterOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = Writer::new(writer_options.clone());
    let parser = parser::Parser::build(bytes)?;
    let mut index: usize = 0;

    writer.write_comment(file_name);
    writer.end_line();
    writer.write(b"bits 16");
    writer.end_line();

    for instruction in parser {
        instruction.write(&mut writer);
        index += instruction.length as usize;
    }

    stdout().write_all(writer.as_slice())?;

    if writer_options.verbose && index < bytes.len() {
        print!(
            "; Warning: {} bytes were not parsed. Bytes:",
            bytes.len() - index,
        );

        for &byte in &bytes[index..] {
            print!(" {:08b}", byte);
        }

        println!();
    }

    Ok(())
}

pub fn run_from_file(
    file_name: &str,
    writer_options: WriterOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_file(file_name)?;
    let base_name = std::path::Path::new(file_name)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    run(base_name, bytes.as_slice(), writer_options)
}
