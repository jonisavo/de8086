# de8086

A 8086/8088 disassembler written in Rust. It is by no means perfect; it can not
disassemble every instruction at the moment.

# CLI

The CLI takes a filename as input, i.e.

```shell script
de8086 ./test/kitchen_sink
```

Outputs:

```
; kitchen_sink

bits 16
mov cx, bx
mov ch, ah
mov dx, bx
...
```

Specifying the `--verbose` flag includes additional output.

```shell script
de8086 ./test/kitchen_sink --verbose
```

Outputs:

```
; kitchen_sink

bits 16
; 10001001 11011001
mov cx, bx
; 10001000 11100101
mov ch, ah
; 10001001 11011010
mov dx, bx
```

# API

de8086 comes with `Parser` and `Writer` structs
for disassembly:

```rust
use de8086::{
    parser::Parser,
    writer::{Writer, WriterOptions},
};
use std::io::{stdout, Write};

fn run(bytes: &[u8]) {
    let mut writer = Writer::new(WriterOptions { verbose: false });
    let parser = Parser::build(bytes).unwrap();

    for instruction in parser {
        instruction.write(&mut writer);
    }

    stdout().write_all(writer.as_slice()).unwrap();
}
```
