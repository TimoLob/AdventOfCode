use std::{char, collections::HashSet, u8};

fn get_priority(c: char) -> i64 {
    if c.is_lowercase() {
        (c as u8 - 'a' as u8) as i64 + 1
    } else {
        (c as u8 - 'A' as u8) as i64 + 27
    }
}

fn solve1(input: &str) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let length = line.len();
        let first = line.get(0..length / 2).expect("First half");
        let second = line.get(length / 2..length).expect("second half");
        let mut set: HashSet<char> = HashSet::new();
        first.chars().for_each(|c| {
            set.insert(c);
        });
        for c in second.chars() {
            if set.contains(&c) {
                let prio = get_priority(c);
                println!("{} : {}", c, prio);
                total += prio;
                break;
            }
        }
    }
    total
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = solve1(input);
    println!("Result 1 : {}", result);
}
