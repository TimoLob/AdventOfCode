# Benchmarks
## Simple solution

Timer precision: 72 ns
day01_bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part_1  399.7 µs      │ 1.068 ms      │ 407 µs        │ 424.3 µs      │ 100     │ 100
╰─ bench_part_2  2.23 ms       │ 2.261 ms      │ 2.236 ms      │ 2.237 ms      │ 100     │ 100


## Slightly optimized

Remove full rotations before going step by step in part 2

Timer precision: 60 ns
day01_bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part_1  403 µs        │ 733.7 µs      │ 410.9 µs      │ 417.3 µs      │ 100     │ 100
╰─ bench_part_2  995 µs        │ 1.039 ms      │ 1.001 ms      │ 1.002 ms      │ 100     │ 100
