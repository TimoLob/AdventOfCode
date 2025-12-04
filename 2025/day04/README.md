# Day 04

## Part 1
1. Create a HashSet of positions with rolls
2. For each, count the number of neighbors

## Part 2
1. Create a HashSet of position with rolls
2. Create a Map from position -> #neighbors
3. Find all rolls with <4 neighbors, add them to a set that acts as queue
4. For each roll in the queue
    - Remove it from the map
    - increase result counter
    - decrese neighbor value for each neighbor by 1
    - if that causes the value to be less than 4, add it to the queue

## Benchmark
Timer precision: 47 ns
```
day04_bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part_1  1.223 ms      │ 2.218 ms      │ 1.237 ms      │ 1.251 ms      │ 100     │ 100
╰─ bench_part_2  3.137 ms      │ 3.793 ms      │ 3.18 ms       │ 3.188 ms      │ 100     │ 100
```
