use criterion::{black_box, criterion_group, criterion_main, Criterion};
use de8086::instructions::mov::{parse_mov_immediate_to_register, parse_mov_memory_to_accumulator, parse_mov_to_register};
use de8086::parser::Parser;
use de8086::writer::Writer;

const KITCHEN_SINK_BYTES: &[u8] = include_bytes!("../test/kitchen_sink");
const BIG_FILE_BYTES: &[u8] = include_bytes!("../test/big_file");

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
    group.bench_function("immediate to register", |b| {
        const BYTES: [u8; 3] = [0b10111100, 0x12, 0x34];
        b.iter(|| parse_mov_immediate_to_register(black_box(&BYTES)))
    });
    group.bench_function("memory to accumulator", |b| {
        const BYTES: [u8; 3] = [0b10100001, 0x12, 0x34];
        b.iter(|| parse_mov_memory_to_accumulator(black_box(&BYTES)))
    });
}

fn benchmark_file(c: &mut Criterion) {
    c.bench_function("parse kitchen sink file", |b| {
        let parser = Parser::build(black_box(KITCHEN_SINK_BYTES)).unwrap();
        b.iter(move || {
            for instruction in parser {
                black_box(instruction);
            }
        })
    });

    c.bench_function("parse big file (1000 elements)", |b| {
        let parser = Parser::build(black_box(KITCHEN_SINK_BYTES)).unwrap();
        b.iter(move || {
            for instruction in parser {
                black_box(instruction);
            }
        })
    });
}

fn benchmark_write(c: &mut Criterion) {
    c.bench_function("write kitchen sink file", |b| {
        let parser = Parser::build(black_box(KITCHEN_SINK_BYTES)).unwrap();
        let mut writer = Writer::new();
        let instructions = &parser.collect::<Vec<_>>();
        b.iter(move || {
            for instruction in instructions {
                instruction.write(&mut writer);
            }
        })
    });

    c.bench_function("write big file (1000 elements)", |b| {
        let parser = Parser::build(black_box(BIG_FILE_BYTES)).unwrap();
        let mut writer = Writer::new();
        let instructions = &parser.collect::<Vec<_>>();
        b.iter(move || {
            for instruction in instructions {
                instruction.write(&mut writer);
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_parse_mov,
    benchmark_file,
    benchmark_write
);
criterion_main!(benches);
