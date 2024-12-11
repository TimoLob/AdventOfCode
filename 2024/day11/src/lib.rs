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

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let mut stones: Vec<Stone> = input
        .split(' ')
        .map(|x| x.parse::<Stone>().unwrap())
        .collect();

    let num_blinks = 25;
    for _ in 0..num_blinks {
        let len = stones.len();
        for i in 0..len {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
            } else if let Some((l, r)) = split_stone(&stone) {
                stones[i] = l;
                stones.push(r);
            } else {
                stones[i] = stone * 2024;
            }
        }
    }

    stones.len().to_string()
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

pub fn part2(input: &str) -> String {
    let input = input.trim();
    let input = input.split(' ').map(|x| x.parse::<Stone>().unwrap());

    let mut stones: HashMap<Stone, usize> = HashMap::new();
    input.for_each(|stone| _ = stones.entry(stone).and_modify(|x| *x += 1).or_insert(1));

    let num_blinks = 75;
    for _blink in 0..num_blinks {
        let mut new_stones: HashMap<Stone, usize> = HashMap::new();
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
    total.to_string()
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
        assert_eq!(result, ""); // Replace with the actual expected result
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
