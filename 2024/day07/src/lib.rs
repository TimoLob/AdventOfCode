#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

fn is_valid_equation(eq: &Equation) -> bool {
    is_valid_sub_equation(eq.result, &eq.numbers)
}

fn is_valid_sub_equation(result: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 0 {
        return result == 0;
    }
    let x = numbers[numbers.len() - 1];
    if result % x == 0 && is_valid_sub_equation(result / x, &numbers[0..numbers.len() - 1]) {
        return true;
    }
    if result - x >= 0 && is_valid_sub_equation(result - x, &numbers[0..numbers.len() - 1]) {
        return true;
    }
    false
}

fn is_valid_equation_part2(eq: &Equation) -> bool {
    is_valid_sub_equation_part2(eq.result, &eq.numbers)
}

fn cut_off(a: i64, b: i64) -> Option<i64> {
    let a = a.to_string();
    let b = b.to_string();
    if a.ends_with(&b) {
        let trimmed = &a[..a.len() - b.len()];
        if trimmed.is_empty() {
            return Some(0);
        }
        return trimmed.parse::<i64>().ok();
    }
    None
}

fn is_valid_sub_equation_part2(result: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 0 {
        return result == 0;
    }
    let x = numbers[numbers.len() - 1];
    if result % x == 0 && is_valid_sub_equation_part2(result / x, &numbers[0..numbers.len() - 1]) {
        return true;
    }
    if result - x >= 0 && is_valid_sub_equation_part2(result - x, &numbers[0..numbers.len() - 1]) {
        return true;
    }
    if let Some(cut) = cut_off(result, x) {
        if is_valid_sub_equation_part2(cut, &numbers[0..numbers.len() - 1]) {
            return true;
        }
    }

    false
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let equations = input.lines().map(|line| {
        let (value, numbers) = line.split_once(':').unwrap();
        let value = value.parse::<i64>().unwrap();
        let numbers: Vec<i64> = numbers
            .trim()
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        Equation {
            result: value,
            numbers,
        }
    });
    let total: i64 = equations
        .filter_map(|eq| {
            if is_valid_equation(&eq) {
                Some(eq.result)
            } else {
                None
            }
        })
        .sum();

    total.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let equations = input.lines().map(|line| {
        let (value, numbers) = line.split_once(':').unwrap();
        let value = value.parse::<i64>().unwrap();
        let numbers: Vec<i64> = numbers
            .trim()
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        Equation {
            result: value,
            numbers,
        }
    });
    let total: i64 = equations
        .filter_map(|eq| {
            if is_valid_equation_part2(&eq) {
                Some(eq.result)
            } else {
                None
            }
        })
        .sum();

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
        assert_eq!(result, "3749"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "11387"); // Replace with the actual expected result
    }
}
