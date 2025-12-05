# Day 05

Parsing is done with [nom](https://docs.rs/nom/latest/nom/index.html) today.


## Part 1
Put all ranges in a Vec, for each ingredient check if it is in any range.

## Part 2

1. Start with empty set.
2. For each new range R:
    - Determine all ranges in the existing set that overlap R
    - Combine all those overlapping ranges with R into one range
    - Remove overlapping ranges from set
    - add new combined range to set
3. Count the length of all ranges in set and return sum

## Tests

1. Part 1 example
2. Part 2 example
3. `is_overlap` function
4. `combine_overlapping_ranges` function

## Benchmark

```
Timer precision: 15 ns
day05_bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part_1  68.63 µs      │ 101.5 µs      │ 78.34 µs      │ 80.74 µs      │ 100     │ 100
╰─ bench_part_2  52.48 µs      │ 60.66 µs      │ 54.73 µs      │ 54.77 µs      │ 100     │ 100

```