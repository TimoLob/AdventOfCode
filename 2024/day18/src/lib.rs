use std::collections::HashSet;

use glam::IVec2;
use pathfinding::prelude::astar;

fn parse(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            IVec2 { x, y }
        })
        .collect()
}

fn successors(pos: &IVec2, width: u32, height: u32, bytes: &HashSet<IVec2>) -> Vec<(IVec2, u32)> {
    let directions = vec![IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];
    let mut succ = Vec::new();
    for dir in directions.iter() {
        let new_pos = pos + dir;
        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x > width as i32 || new_pos.y > height as i32 {
            continue;
        }
        if bytes.contains(&new_pos) {
            continue;
        }
        succ.push((new_pos, 1));
    }
    succ
}

fn find_path(width: u32, height: u32, bytes: &Vec<IVec2>, fallen: usize) -> Option<u32> {
    let start = IVec2 { x: 0, y: 0 };
    let target = IVec2 {
        x: width as i32,
        y: height as i32,
    };
    let bytes = bytes
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx < fallen)
        .map(|(_, x)| *x)
        .collect::<HashSet<IVec2>>();
    let res = astar(
        &start,
        |p| successors(p, width, height, &bytes),
        |p| target.distance_squared(*p) as u32,
        |p| *p == target,
    );
    if let Some(res) = res {
        return Some(res.1);
    }
    None
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let bytes = parse(input);
    let res = find_path(70, 70, &bytes, 1024);
    res.unwrap().to_string()
}

fn find_first_blocking(width: u32, height: u32, bytes: &Vec<IVec2>) -> IVec2 {
    let mut min = 0;
    let mut max = bytes.len();

    loop {
        let mid = (max + min) / 2;
        if mid == min || max == min {
            break;
        }
        let res = find_path(width, height, bytes, mid);
        dbg!("-");
        match res {
            Some(_) => min = mid,
            None => max = mid,
        };
    }
    bytes[min]
}

pub fn part2(input: &str) -> String {
    let input = input.trim();

    let bytes = parse(input);

    let res = find_first_blocking(70, 70, &bytes);
    res.to_string()
}

pub fn part2_challenge(input: &str) -> String {
    let input = input.trim();

    let bytes = parse(input);

    let res = find_first_blocking(212, 212, &bytes);

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");

        let input = input.trim();

        let bytes = parse(input);
        let result = find_path(6, 6, &bytes, 12);
        assert_eq!(result.unwrap(), 22); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");

        let input = input.trim();
        let bytes = parse(input);

        let res = find_first_blocking(6, 6, &bytes);
        assert_eq!(res, IVec2 { x: 6, y: 1 }); // Replace with the actual expected result
    }
}
