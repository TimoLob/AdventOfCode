# Day 09


## Part 1

Check all pairwise rectangles.

## Part 2

1. Compress coordinate space to only used coords
2. Check all pairwise rectangles
    - If it is larger than the current largest one, check all coordinates in that rectangle whether they are in the polygon
    - If they are, save candidate
     

## Bench
```
Timer precision: 17 ns
day09_bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part_1  90.9 µs       │ 109.7 µs      │ 97.69 µs      │ 97.1 µs       │ 100     │ 100
╰─ bench_part_2  380 ms        │ 407.2 ms      │ 381.5 ms      │ 383 ms        │ 100     │ 100
```