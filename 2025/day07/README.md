# Day 07
## Part 1

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


## Part 2

Very similar to part 1. Except lasers are stored in a vector as a struct with `position` and `universes`. Universes is the number of universes that can reach that position.  
Initially that vector is `{ position of S, 1}`.  
When part 1 would insert a laser into the HashSet, instead check if that laser already exists in the next Vec of lasers. If it does, add the current lasers universe count to that. If not, add the new laser with the current universe count to the new laser vector.
Return the sum of universes in the final vector.

## Benchmark
Part 2 faster than part 1. Probably because a vector is faster than a HashSet at this sample size.
I could probably improve the performance even more with some kind of sorted data structure for the lasers (for both part 1 and 2). Currently part 1 relies on a HashSet to keep uniqueness, part 2 searches the whole vector for every insertion.
Since the input is only ~140 chars long per line, I could also use a fixed size vector for instant lookup.

```
Timer precision: 10 ns
day07_bench      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_part_1  131.9 µs      │ 544.8 µs      │ 159.3 µs      │ 202 µs        │ 100     │ 100
╰─ bench_part_2  101.3 µs      │ 127.5 µs      │ 104.4 µs      │ 105.9 µs      │ 100     │ 100
```