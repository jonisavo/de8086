pub struct Writer {
    buffer: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        let mut buffer = Vec::new();

        buffer.reserve(32);

        Self { buffer }
    }

    pub fn write(&mut self, bytes: &[u8]) -> &mut Self {
        self.buffer.extend_from_slice(bytes);

        self
    }

    pub fn write_string(&mut self, string: &str) -> &mut Self {
        self.write(string.as_bytes())
    }

    pub fn write_comma_separator(&mut self) -> &mut Self {
        self.write(b", ")
    }

    pub fn write_comment(&mut self, comment: &str) -> &mut Self {
        self.write(b"; ").write_string(comment).end_line()
    }

    pub fn end_line(&mut self) -> &mut Self {
        self.buffer.push(b'\n');

        self
    }

    pub fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }
}