use regex::Regex;

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let re = Regex::new(r"mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)").unwrap();
    let caps = re.find_iter(input);
    let mut total = 0;
    for cap in caps {
        let instruction = cap.as_str();
        let numbers = &instruction[4..instruction.len() - 1];
        let (x, y) = numbers
            .split_once(',')
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .unwrap();
        total += x * y;
    }
    total.to_string()
}
#[derive(Debug, PartialEq, Eq)]
enum Status {
    ENABLED,
    DISABLED,
}

pub fn part2(input: &str) -> String {
    let input = input.trim();
    let re = Regex::new(r"mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)|do\(\)|don't\(\)").unwrap();
    let caps = re.find_iter(input);
    let mut total = 0;
    let mut status: Status = Status::ENABLED;
    for cap in caps {
        let instruction = cap.as_str();
        match instruction {
            "do()" => status = Status::ENABLED,
            "don't()" => status = Status::DISABLED,
            _ => {
                let numbers = &instruction[4..instruction.len() - 1];
                let (x, y) = numbers
                    .split_once(',')
                    .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                    .unwrap();
                if status == Status::ENABLED {
                    total += x * y;
                }
            }
        }
    }
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
        assert_eq!(result, "161");
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example2.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "48");
    }
}
