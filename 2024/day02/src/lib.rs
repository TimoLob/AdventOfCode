fn verify_report(report: &Vec<i32>) -> i32 {
    if report.len() == 1 {
        return 1;
    }
    if report.len() == 0 {
        println!("Unexpected length 0 report");
        return 0;
    }
    if report[0] - report[1] > 0 {
        // Increasing
        for index in 0..report.len() - 1 {
            let distance = report[index] - report[index + 1];
            if distance < 0 {
                return 0;
            }
            if distance < 1 || distance > 3 {
                return 0;
            }
        }
    } else {
        // Decreasing
        for index in 0..report.len() - 1 {
            let distance = report[index + 1] - report[index];
            if distance < 0 {
                return 0;
            }
            if distance < 1 || distance > 3 {
                return 0;
            }
        }
    }
    1
}

fn verify_report_part2(report: &Vec<i32>) -> i32 {
    if verify_report(report) == 1 {
        return 1;
    }
    // Not a fan of this O(n^2) solution, but it finishes instantly
    for index in 0..report.len() {
        let mut new_report: Vec<i32> = Vec::new();
        for i in 0..report.len() {
            if i != index {
                new_report.push(report[i]);
            }
        }
        if verify_report(&new_report) == 1 {
            return 1;
        }
    }
    0
}
pub fn part1(input: &str) -> String {
    let input = input.trim();
    let lines = input.split('\n');
    let reports: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let result = reports
        .iter()
        .fold(0, |acc: i32, report| verify_report(report) + acc);
    result.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let lines = input.split('\n');
    let reports: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let result = reports
        .iter()
        .fold(0, |acc: i32, report| verify_report_part2(report) + acc);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "2"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "4"); // Replace with the actual expected result
    }
}
