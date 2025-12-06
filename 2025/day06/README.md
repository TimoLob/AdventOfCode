# Day 06

Just a parsing challenge. 

## Part 1
Using a nom parser to deal with weird spacing.

## Part 2

Hard-coded parsing. Numbers into a 2d vector (spaces become 0). Operators were parsed into a vector that contains the operators and one that contains how many digits the numbers above it have.

Going from left-to-right read number from top to bottom, ignoring zeroes (No zeroes in input made this easy) and apply the operator.

Actually faster than part 1.

## Benchmark
```
Timer precision: 10 ns
day06_bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part_1  53.66 µs      │ 79.53 µs      │ 61.24 µs      │ 60.97 µs      │ 100     │ 100
╰─ bench_part_2  18.44 µs      │ 34.91 µs      │ 19.11 µs      │ 20.51 µs      │ 100     │ 100
```