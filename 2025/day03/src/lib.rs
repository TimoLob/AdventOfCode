use std::{collections::BinaryHeap};



pub fn part1(input: &str) -> String {
    /*
    Find the biggest value in the current bank. Then, for each occurence of that biggest value, find the biggest value that comes after it.
    If there are multiple options, the max joltage is biggest value*10 + max(next_biggest_options)
    If there are no options (biggest value is only at the end), start again from the second biggest batterie
    */

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
            });
            
            let second_biggest_options = biggest_indices.filter_map(|start_idx| {
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
    /*
        hint from : https://www.reddit.com/r/adventofcode/comments/1pcxj6c/comment/ns16nrk
    */
    let input = input.trim();
    let banks = input.lines().map(|line| {
        let digits = line.chars().map(|c| c.to_digit(10).expect("Failed to parse char into digit.") as u64).collect::<Vec<u64>>();
        let mut picked_digits:Vec<u64> = Vec::with_capacity(12);
        let mut current_start =0;
        let mut current_slice_len = 0;
        for _ in 0..12 {
            let end = current_start + (digits[current_start..digits.len()].len() - (11-current_slice_len));

            let slice = &digits[current_start..end];
            let max = *slice.iter().max().unwrap();

            let pos = slice.iter().position(|&v| v==max).unwrap();
            current_start += pos+1;
            picked_digits.push(max);
            current_slice_len+=1;
        }
        let mut exp = 0;
        let mut total = 0;
        for d in picked_digits.iter().rev() {
            total += d * 10u64.pow(exp);
            exp+=1;
        }
        total
    });

    banks.sum::<u64>().to_string()
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
        assert_eq!(result, "3121910778619"); // Replace with the actual expected result
    }
}
