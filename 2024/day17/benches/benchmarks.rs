use day17::part1;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn bench_part_1() {
    part1(divan::black_box(include_str!("../input.txt",)));
}
