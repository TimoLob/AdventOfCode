use std::{char, collections::HashSet, u8};
fn get_priority(c: char) -> i64 {
    if c.is_lowercase() {
        (c as u8 - 'a' as u8) as i64 + 1
    } else {
        (c as u8 - 'A' as u8) as i64 + 27
    }
}
fn solve1(input: &str) -> i64 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|tuple| {
            let mut sets: Vec<HashSet<char>> = vec![HashSet::new(), HashSet::new()];
            for i in 0..2 {
                for c in tuple[i].chars() {
                    sets[i].insert(c);
                }
            }
            for c in tuple[2].chars() {
                if sets[0].contains(&c) && sets[1].contains(&c) {
                    return get_priority(c);
                }
            }
            unreachable!()
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = solve1(input);
    println!("Result 1 : {}", result);
}
