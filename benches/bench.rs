use criterion::{black_box, criterion_group, criterion_main, Criterion};
use de8086::instructions::mov::{IMMEDIATE_TO_REGISTER, MEMORY_TO_ACCUMULATOR, TO_REGISTER};
use de8086::parser::Parser;
use de8086::writer::{Writer, WriterOptions};
use de8086::Instruction;

const KITCHEN_SINK_BYTES: &[u8] = include_bytes!("../test/kitchen_sink");
const EVIL_FILE_BYTES: &[u8] = include_bytes!("../test/evil_file");
const MOV_FILE_BYTES: &[u8] = include_bytes!("../test/mov_file");

fn benchmark_parse_mov(c: &mut Criterion) {
    let mut group = c.benchmark_group("mov");
    let mut instruction = Instruction::EMPTY;
    group.bench_function("reg to reg", |b| {
        const BYTES: [u8; 2] = [0b10001001, 0b11011100];
        b.iter(|| TO_REGISTER.parse(black_box(&BYTES), &mut instruction))
    });
    group.bench_function("bp + si + constant calculation", |b| {
        const BYTES: [u8; 4] = [0b10001001, 0b10011010, 0xab, 0xcd];
        b.iter(|| TO_REGISTER.parse(black_box(&BYTES), &mut instruction))
    });
    group.bench_function("immediate to register", |b| {
        const BYTES: [u8; 3] = [0b10111100, 0x12, 0x34];
        b.iter(|| IMMEDIATE_TO_REGISTER.parse(black_box(&BYTES), &mut instruction))
    });
    group.bench_function("memory to accumulator", |b| {
        const BYTES: [u8; 3] = [0b10100001, 0x12, 0x34];
        b.iter(|| MEMORY_TO_ACCUMULATOR.parse(black_box(&BYTES), &mut instruction))
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

    c.bench_function("parse evil file", |b| {
        let parser = Parser::build(black_box(EVIL_FILE_BYTES)).unwrap();

        b.iter(move || {
            for instruction in parser {
                black_box(instruction);
            }
        })
    });

    c.bench_function("parse mov file (1000 elements)", |b| {
        let parser = Parser::build(black_box(MOV_FILE_BYTES)).unwrap();
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
        let mut writer = Writer::new(WriterOptions { verbose: false });
        let instructions = &parser.collect::<Vec<_>>();
        b.iter(move || {
            for instruction in instructions {
                instruction.write(&mut writer);
            }
        })
    });

    c.bench_function("write evil file", |b| {
        let parser = Parser::build(black_box(EVIL_FILE_BYTES)).unwrap();
        let mut writer = Writer::new(WriterOptions { verbose: false });
        let instructions = &parser.collect::<Vec<_>>();
        b.iter(move || {
            for instruction in instructions {
                instruction.write(&mut writer);
            }
        })
    });

    c.bench_function("write mov file (1000 elements)", |b| {
        let parser = Parser::build(black_box(MOV_FILE_BYTES)).unwrap();
        let mut writer = Writer::new(WriterOptions { verbose: false });
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
