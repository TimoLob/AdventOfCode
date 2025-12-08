# Day 08

## Part 1

1. Parse boxes into a vec of IVec3
2. Calculate pairwise distances for each pair of boxes
3. Sort distances
4. Create a Vec of sets (Circuits), each Circuit contains exactly one junction box
5. From smallest distance between A and B, repeat N times:
    - Find the circuits that A and B belong to
    - Move all junction boxes from set B to set A.
    - Remove set B from vec of sets
6. Find 3 largest sets and multiply their size


## Part 2

Do part 1, except keep going until only one circuit remains. If it does, multiply and return the x-coords of the two last merged junction boxes.

## Benchmark


```
Timer precision: 27 ns
day08_bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part_1  22.55 ms      │ 33.55 ms      │ 22.86 ms      │ 23.14 ms      │ 100     │ 100
╰─ bench_part_2  23.09 ms      │ 27 ms         │ 23.41 ms      │ 24.15 ms      │ 100     │ 100
```