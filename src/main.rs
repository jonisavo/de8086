use de8086::{run_from_file, writer::WriterOptions};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    let verbose = args.len() > 2 && (args[2] == "-v" || args[2] == "--verbose");

    run_from_file(&args[1], WriterOptions { verbose })
}
