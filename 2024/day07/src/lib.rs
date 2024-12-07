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
    is_valid_sub_equation_part2(eq.result, 0, &eq.numbers)
}

fn concat(a: i64, b: i64) -> i64 {
    format!("{a}{b}").parse::<i64>().unwrap()
}

fn is_valid_sub_equation_part2(result: i64, current: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 0 {
        return result == current;
    }
    let x = numbers[0];
    let mul = current * x;
    if mul <= result && is_valid_sub_equation_part2(result, mul, &numbers[1..numbers.len()]) {
        return true;
    }
    let add = current + x;
    if add <= result && is_valid_sub_equation_part2(result, add, &numbers[1..numbers.len()]) {
        return true;
    }
    let concat = concat(current, x);
    if concat <= result && is_valid_sub_equation_part2(result, concat, &numbers[1..numbers.len()]) {
        return true;
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
