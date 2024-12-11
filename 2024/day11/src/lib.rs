use std::{collections::HashMap, usize};

type Stone = u64;

fn split_stone(stone: &Stone) -> Option<(Stone, Stone)> {
    let num_digits = (*stone as f64).log10().floor() as u32 + 1;
    if num_digits % 2 != 0 {
        return None;
    }
    let divisor: u64 = 10u64.pow(num_digits / 2);
    let lower_half = stone % divisor;
    let upper_half = stone / divisor;
    Some((upper_half, lower_half))
}

enum Result {
    SingleStone(Stone),
    TwoStones(Stone, Stone),
}

fn process_stone(stone: &Stone) -> Result {
    if *stone == 0 {
        Result::SingleStone(1)
    } else if let Some((l, r)) = split_stone(stone) {
        Result::TwoStones(l, r)
    } else {
        Result::SingleStone(stone * 2024)
    }
}

fn blink(input: &Vec<Stone>, num_blinks: usize) -> usize {
    let mut stones: HashMap<Stone, usize> = HashMap::with_capacity(input.len());
    input
        .iter()
        .for_each(|stone| _ = stones.entry(*stone).and_modify(|x| *x += 1).or_insert(1));

    for _blink in 0..num_blinks {
        let mut new_stones: HashMap<Stone, usize> = HashMap::with_capacity(stones.len() * 2);
        stones.iter().for_each(|(stone, number)| {
            let r = process_stone(stone);
            match r {
                Result::SingleStone(stone) => {
                    _ = new_stones
                        .entry(stone)
                        .and_modify(|x| *x += number)
                        .or_insert(*number);
                }
                Result::TwoStones(left, right) => {
                    _ = new_stones
                        .entry(left)
                        .and_modify(|x| *x += number)
                        .or_insert(*number);
                    _ = new_stones
                        .entry(right)
                        .and_modify(|x| *x += number)
                        .or_insert(*number)
                }
            };
        });
        stones = new_stones;
    }
    let total: usize = stones.iter().map(|(_, num)| *num).sum();
    total
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let input = input
        .split(' ')
        .map(|x| x.parse::<Stone>().unwrap())
        .collect::<Vec<Stone>>();
    blink(&input, 25).to_string()
}

pub fn part2(input: &str) -> String {
    let input = input.trim();
    let input = input
        .split(' ')
        .map(|x| x.parse::<Stone>().unwrap())
        .collect::<Vec<Stone>>();
    blink(&input, 75).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "55312"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "65601038650482"); // Replace with the actual expected result
    }

    #[test]
    fn test_even_length() {
        assert_eq!(split_stone(&1234), Some((12, 34)));
        assert_eq!(split_stone(&567890), Some((567, 890)));
        assert_eq!(split_stone(&1001), Some((10, 01)));
    }

    #[test]
    fn test_uneven_length() {
        assert_eq!(split_stone(&123), None);
        assert_eq!(split_stone(&12345), None);
        assert_eq!(split_stone(&9), None);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(split_stone(&0), None); // Single digit
        assert_eq!(split_stone(&10), Some((1, 0)));
        assert_eq!(split_stone(&100000), Some((100, 000)));
    }
}
