# Day 07
## Part 1

Active lasers are represented by a bool array of size 150 (magic number since input is ~142 chars long).

1. For each line of splitters
    - Create new laser array of 150 zeroes (`[false; 150]`)
    - For each `(pos, universes)` in `lasers.enumerate()`
        - Skip if `universes==0`
        - Binary Search current line of splitters for a splitter at `pos`
        - if splitter : `new_lasers[pos(+/-)1] = true`; increase split counter
        - if not : `new_lasers[pos] += true` (Laser goes straight)
    - active lasers := new lasers
2. Return split counter


## Part 2

Active lasers are represented by an usize array of size 150.
Each entry corresponds to the number of universes for the active laser at that index. Initially this array is empty except for a `1` at index S.

1. For each line of splitters
    - Create new laser array of 150 zeroes (`[0; 150]`)
    - For each `(pos, universes)` in `lasers.enumerate()`
        - Skip if `universes==0`
        - Binary Search current line of splitters for a splitter at `pos`
        - if splitter : `new_lasers[pos(+/-)1] += universes`
        - if not : `new_lasers[pos] += universes` (Laser goes straight)
    - active lasers := new lasers
2. Sum active lasers

## Benchmark
Part 2 way faster than part 1 due to no hashing and instant lookput in the lasers data structure.


```
Timer precision: 47 ns
day07_bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part_1  35.67 µs      │ 118.8 µs      │ 43.18 µs      │ 48.78 µs      │ 100     │ 100
╰─ bench_part_2  32.32 µs      │ 57.89 µs      │ 35.12 µs      │ 36.4 µs       │ 100     │ 100

```

## Part 1 (old)

1. Inititialize a Set of active Lasers with the X position of 'S'
2. Parse the input into a `Vec<Vec<usize>>`. 
    - For each line of input, create a `Vec<usize>` that contains the X position of splitters in that line.
3. For each line of splitters
    - Initialize a new empty set of next lasers
    - For each active laser at position `l`
        - Binary Search the current line of splitters for `l`
        - If there is not splitter at that position : Copy `l` to the new set of lasers
        - If there is one: Insert `l+1` and `l-1` into the new set of lasers; Increment split counter
    - Replace active lasers with new lasers
4. Return split counter
