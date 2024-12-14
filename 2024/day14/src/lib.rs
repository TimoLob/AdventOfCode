use std::collections::HashSet;

use glam::IVec2;
use regex::Regex;

#[derive(Debug)]
struct Robot {
    pos: IVec2,
    vel: IVec2,
}

fn parse(input: &str) -> Vec<Robot> {
    let robot_regex = Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = robot_regex.captures(line).unwrap();
            let p_x: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
            let p_y: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
            let v_x: i32 = captures.get(3).unwrap().as_str().parse().unwrap();
            let v_y: i32 = captures.get(4).unwrap().as_str().parse().unwrap();
            let pos = IVec2::new(p_x, p_y);
            let vel = IVec2::new(v_x, v_y);
            Robot { pos, vel }
        })
        .collect()
}

fn step(robots: &mut Vec<Robot>, width: i32, height: i32) {
    robots.iter_mut().for_each(|r| {
        r.pos += r.vel;
        r.pos.x = (r.pos.x + width) % width;
        r.pos.y = (r.pos.y + height) % height;
    });
}

enum Quadrant {
    NE,
    NW,
    SE,
    SW,
    None,
}
fn simulate_part1(robots: &mut Vec<Robot>, width: i32, height: i32, num_steps: usize) -> usize {
    for _ in 0..num_steps {
        step(robots, width, height);
    }

    let quadrants = robots.iter().map(|r| match r.pos {
        IVec2 { x, y } if x < width / 2 && y < height / 2 => Quadrant::NW,
        IVec2 { x, y } if x < width / 2 && y > height / 2 => Quadrant::SW,
        IVec2 { x, y } if x > width / 2 && y < height / 2 => Quadrant::NE,
        IVec2 { x, y } if x > width / 2 && y > height / 2 => Quadrant::SE,

        IVec2 { x: _x, y: _y } => Quadrant::None,
    });
    let mut ne = 0;
    let mut nw = 0;
    let mut se = 0;
    let mut sw = 0;
    quadrants.for_each(|q| match q {
        Quadrant::NE => ne += 1,
        Quadrant::NW => nw += 1,
        Quadrant::SE => se += 1,
        Quadrant::SW => sw += 1,
        Quadrant::None => {}
    });
    ne * nw * se * sw
}

fn print_robots(robots: &Vec<Robot>, width: i32, height: i32) {
    let mut positions: HashSet<IVec2> = HashSet::new();
    robots.iter().for_each(|r| _ = positions.insert(r.pos));
    for y in 0..height {
        for x in 0..width {
            if positions.contains(&IVec2 { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn simulate_part2(robots: &mut Vec<Robot>, width: i32, height: i32, num_steps: usize) {
    for _ in 0..num_steps {
        step(robots, width, height);
    }
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let mut robots = parse(input);
    simulate_part1(&mut robots, 101, 103, 100).to_string()
}
// Assumption : Christmas Tree has no overlapping robots
// Print all indices where there are no overlapping robots, then manually check them
pub fn part2(input: &str) -> String {
    let input = input.trim();

    let mut robots = parse(input);
    let steps_per_iter = 1;
    let mut positions: HashSet<IVec2> = HashSet::new();

    // Initially this was a very large number to get possible times
    for i in 0..7847 {
        simulate_part2(&mut robots, 101, 103, steps_per_iter);
        positions.clear();
        robots.iter().for_each(|r| _ = positions.insert(r.pos));
        if robots.len() == positions.len() {
            println!("i:{}", i);
        }
    }
    print_robots(&robots, 101, 103);
    0.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");

        let mut robots = parse(&input);

        let result = simulate_part1(&mut robots, 11, 7, 100);
        assert_eq!(result, 12); // Replace with the actual expected result
    }
}
