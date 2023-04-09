use std::io::{BufWriter, Read, Stdout, Write};

pub fn read_file(filename: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(filename)?;

    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    return Ok(buffer);
}

pub fn write_comment(writer: &mut BufWriter<Stdout>, comment: &str) {
    writer.write_all(b"; ").unwrap();
    writer.write_all(comment.as_bytes()).unwrap();
    writer.write_all(b"\n").unwrap();
}

pub fn write_line(writer: &mut BufWriter<Stdout>, line: &str) {
    writer.write_all(line.as_bytes()).unwrap();
    writer.write_all(b"\n").unwrap();
}
