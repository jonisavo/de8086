use de8086::run_from_file;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    run_from_file(&args[1])
}
