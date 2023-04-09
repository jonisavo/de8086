use criterion::{black_box, criterion_group, criterion_main, Criterion};
use de8086::instructions::mov::parse_mov_to_register;
use de8086::parser::Parser;

const EFFECTIVE_ASM_BYTES: &[u8] = include_bytes!("../test/effective");

fn benchmark_parse_mov(c: &mut Criterion) {
    let mut group = c.benchmark_group("mov memory/reg to/from register");
    group.bench_function("reg to reg", |b| {
        const BYTES: [u8; 2] = [0b10001001, 0b11011100];
        b.iter(|| parse_mov_to_register(black_box(&BYTES)))
    });
    group.bench_function("memory to reg", |b| {
        const BYTES: [u8; 4] = [0b10001011, 0b00011110, 0x12, 0x34];
        b.iter(|| parse_mov_to_register(black_box(&BYTES)))
    });
    group.bench_function("memory from reg", |b| {
        const BYTES: [u8; 4] = [0b10001001, 0b00011110, 0x12, 0x34];
        b.iter(|| parse_mov_to_register(black_box(&BYTES)))
    });
    group.bench_function("bp + si + constant calculation", |b| {
        const BYTES: [u8; 4] = [0b10001001, 0b10011010, 0xab, 0xcd];
        b.iter(|| parse_mov_to_register(black_box(&BYTES)))
    });
}

fn benchmark_file(c: &mut Criterion) {
    c.bench_function("parse bytes", |b| {
        b.iter(|| {
            let parser = Parser::build(black_box(EFFECTIVE_ASM_BYTES)).unwrap();

            for instruction in parser {
                black_box(instruction);
            }
        })
    });
}

criterion_group!(benches, benchmark_parse_mov, benchmark_file);
criterion_main!(benches);
