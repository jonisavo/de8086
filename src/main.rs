pub mod file;
pub mod instructions;
pub mod parser;
pub mod writer;

use file::read_file;
use instructions::parse::parse;
use std::{env, path::Path};
use writer::{write_comment, write_line};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    let bytes = read_file(&args[1])?;
    let base_name = Path::new(&args[1])
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let mut stdout_writer = std::io::BufWriter::new(std::io::stdout());

    write_comment(&mut stdout_writer, base_name);
    write_line(&mut stdout_writer, "");
    write_line(&mut stdout_writer, "bits 16");
    write_line(&mut stdout_writer, "");

    for (_i, byte) in bytes.chunks_exact(2).enumerate() {
        let instruction = (byte[0] as u16) << 8 | byte[1] as u16;
        let parsed_instruction = parse(instruction).unwrap();
        parsed_instruction.write(&mut stdout_writer);
    }

    return Ok(());
}
