use std::io::{BufWriter, Write};

pub fn write_comment<T: std::io::Write>(writer: &mut BufWriter<T>, comment: &str) {
    writer.write_all(b"; ").unwrap();
    writer.write_all(comment.as_bytes()).unwrap();
    writer.write_all(b"\n").unwrap();
}

pub fn write_line<T: std::io::Write>(writer: &mut BufWriter<T>, line: &str) {
    writer.write_all(line.as_bytes()).unwrap();
    writer.write_all(b"\n").unwrap();
}
