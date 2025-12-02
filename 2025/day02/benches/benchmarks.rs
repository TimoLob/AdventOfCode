use day02::{part1, part2};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn bench_part_1() {
    part1(divan::black_box(include_str!("../input.txt",)));
}

#[divan::bench]
fn bench_part_2() {
    part2(divan::black_box(include_str!("../input.txt",)));
}
