use std::collections::BinaryHeap;
use log::info;



pub fn part1(input: &str) -> String {
    let input = input.trim();
    let banks = input.lines().map(|line| {
        let digits = line.chars().map(|c| c.to_digit(10).expect("Failed to parse char into digit.")).collect::<Vec<u32>>();
        let mut heap = BinaryHeap::from(digits.clone());
        loop {
            let biggest = heap.pop().unwrap();
            let biggest_indices = digits.iter().enumerate().filter_map(|(i,&v)| {
                if v == biggest {
                    Some(i)
                }
                else {
                    None
                }
            }).collect::<Vec<usize>>();
            
            let second_biggest_options = biggest_indices.iter().filter_map(|start_idx| {
                digits[start_idx+1..digits.len()].iter().max()

            }).collect::<Vec<&u32>>();
            if second_biggest_options.len() > 0 {
                return biggest*10 + *second_biggest_options.iter().max().unwrap();
            }
        }        
    });

    banks.sum::<u32>().to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "357"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, ""); // Replace with the actual expected result
    }
}
