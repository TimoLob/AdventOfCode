use regex::Regex;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    dir: Direction,
    steps: i64,
}

#[derive(Debug)]
struct Lock {
    size: i64,
    current: i64,
}

impl Lock {
    fn apply(&mut self, rot: Rotation) {
        match rot.dir {
            Direction::Left => self.current -= rot.steps,
            Direction::Right => self.current += rot.steps,
        };
        self.current = self.current.rem_euclid(self.size);
    }

    fn apply_and_count_zeroes(&mut self, rot: &Rotation) -> i64 {
        let sign = match rot.dir {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        let mut zeroes = 0;
        for _ in 0..rot.steps {
            self.current += sign;
            self.current = self.current.rem_euclid(self.size);
            if self.current == 0 {
                zeroes += 1
            
        }

        zeroes
    }
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let mut lock = Lock {
        size: 100,
        current: 50,
    };
    let re = Regex::new(r"([LR])(\d+)").unwrap();
    let mut key = 0i64;
    let rotations = input.lines().map(|line| {
        let cap = re
            .captures(line)
            .expect("Expected properly formatted line.");
        let dir = &cap[1];
        let steps = &cap[2];
        Rotation {
            dir: match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => {
                    panic!()
                }
            },
            steps: steps.parse::<i64>().expect("Failed to parse steps"),
        }
    });
    for rot in rotations {
        lock.apply(rot);
        if lock.current == 0 {
            key += 1;
        }
    }

    key.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let mut lock = Lock {
        size: 100,
        current: 50,
    };
    let re = Regex::new(r"([LR])(\d+)").unwrap();
    let mut key = 0i64;
    let rotations = input.lines().map(|line| {
        let cap = re
            .captures(line)
            .expect("Expected properly formatted line.");
        let dir = &cap[1];
        let steps = &cap[2];
        Rotation {
            dir: match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => {
                    panic!()
                }
            },
            steps: steps.parse::<i64>().expect("Failed to parse steps"),
        }
    });
    for rot in rotations {
        key += lock.apply_and_count_zeroes(&rot);
    }

    key.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "3"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "6"); // Replace with the actual expected result
    }

    #[test]
    fn rot_1000() {
        let mut lock = Lock {
            size: 100,
            current: 50,
        };
        let rot = Rotation {
            dir: Direction::Right,
            steps: 1000,
        };
        let clicks = lock.apply_and_count_zeroes(&rot);
        assert_eq!(clicks, 10);
        assert_eq!(lock.current, 50);
    }
}
