# Day 11

## Part 1


## Part 2

Do part 1 in substeps
1. SVR -> DAC -> FFT -> OUT
2. SVR -> FFT -> DAC -> OUT

Multiply paths and add both options (one of them will be 0 because no cycles).

## Benchmark
```
Timer precision: 17 ns
day11_bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part_1  208.2 µs      │ 301.7 µs      │ 231.8 µs      │ 230.5 µs      │ 100     │ 100
╰─ bench_part_2  297 µs        │ 333.5 µs      │ 304.4 µs      │ 306.6 µs      │ 100     │ 100
```