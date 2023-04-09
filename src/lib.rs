pub mod instructions;
pub use crate::instructions::description::DESCRIPTIONS;
pub use crate::instructions::instruction::Instruction;
mod io;
pub mod parser;

use io::{read_file, write_comment, write_line};
use std::io::{stdout, BufWriter};

pub fn run(file_name: &str, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout_writer = BufWriter::new(stdout());
    let parser = parser::Parser::build(bytes)?;

    write_comment(&mut stdout_writer, file_name);
    write_line(&mut stdout_writer, "");
    write_line(&mut stdout_writer, "bits 16");
    write_line(&mut stdout_writer, "");

    for instruction in parser {
        instruction.write(&mut stdout_writer);
    }

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
