use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Antenna = glam::IVec2;
type Frequency = char;

fn parse(input: &str) -> (IVec2, HashMap<Frequency, Vec<Antenna>>) {
    let input = input.trim();
    let mut antennas: HashMap<Frequency, Vec<Antenna>> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                let antenna = Antenna {
                    x: x as i32,
                    y: y as i32,
                };
                antennas
                    .entry(c)
                    .and_modify(|v| v.push(antenna))
                    .or_insert(vec![antenna]);
            }
        });
    });
    let x_max = input.lines().next().unwrap().len();
    let y_max = input.lines().count();
    let bounds = IVec2 {
        x: x_max as i32,
        y: y_max as i32,
    };
    (bounds, antennas)
}

pub fn part1(input: &str) -> String {
    let (bounds, antennas) = parse(input);
    let mut antinodes: HashSet<IVec2> = HashSet::new();
    for (_, antennas) in antennas.iter() {
        let perms = antennas.iter().permutations(2);
        perms.for_each(|v| {
            let a = v[0];
            let b = v[1];
            let dist = a - b;
            let antinode = a + dist;
            if (0..bounds.x).contains(&antinode.x) && (0..bounds.y).contains(&antinode.y) {
                antinodes.insert(antinode);
            }
        });
    }
    antinodes.len().to_string()
}
pub fn part2(input: &str) -> String {
    let (bounds, antennas) = parse(input);
    let mut antinodes: HashSet<IVec2> = HashSet::new();
    for (_, antennas) in antennas.iter() {
        let perms = antennas.iter().permutations(2);
        perms.for_each(|v| {
            let a = v[0];
            let b = v[1];
            let dist = a - b;
            let mut current = *a;
            while (0..bounds.x).contains(&current.x) && (0..bounds.y).contains(&current.y) {
                antinodes.insert(current);
                current.x += dist.x;
                current.y += dist.y;
            }
        });
    }
    antinodes.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "14"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "34"); // Replace with the actual expected result
    }
}
