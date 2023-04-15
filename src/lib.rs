pub mod instructions;
pub use crate::instructions::instruction::Instruction;
pub mod parser;
pub mod writer;

use std::io::{stdout, Read, Write};
use writer::Writer;

fn read_file(filename: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(filename)?;

    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    return Ok(buffer);
}

pub fn run(file_name: &str, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = Writer::new();
    let parser = parser::Parser::build(bytes)?;

    writer.write_comment(file_name);
    writer.end_line();
    writer.write(b"bits 16");
    writer.end_line();

    for instruction in parser {
        instruction.write(&mut writer);
    }

    stdout().write_all(writer.as_slice()).unwrap();

    Ok(())
}

pub fn run_from_file(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_file(file_name)?;
    let base_name = std::path::Path::new(file_name)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    run(base_name, bytes.as_slice())
}
