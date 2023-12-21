use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    start: Point,
}
impl Grid {
    fn get(&self, point: &Point) -> Tile {
        self.tiles[point.y][point.x]
    }
}

fn step(
    pos: Point,
    grid: &Grid,
    steps_remaining: usize,
    visited: &mut HashSet<(Point, usize)>,
) -> usize {
    if visited.contains(&(pos, steps_remaining)) {
        return 0;
    }
    visited.insert((pos, steps_remaining));
    if steps_remaining == 0 {
        return 1;
    }
    let mut count = 0;
    let mut new_pos = pos;
    new_pos.x += 1;
    if new_pos.x < grid.width && grid.get(&new_pos) == Tile::Plot {
        count += step(new_pos, grid, steps_remaining - 1, visited);
    }
    let mut new_pos = pos;
    new_pos.x -= 1;
    if new_pos.x < grid.width && grid.get(&new_pos) == Tile::Plot {
        count += step(new_pos, grid, steps_remaining - 1, visited);
    }
    let mut new_pos = pos;
    new_pos.y += 1;
    if new_pos.y < grid.height && grid.get(&new_pos) == Tile::Plot {
        count += step(new_pos, grid, steps_remaining - 1, visited);
    }
    let mut new_pos = pos;
    new_pos.y -= 1;
    if new_pos.y < grid.height && grid.get(&new_pos) == Tile::Plot {
        count += step(new_pos, grid, steps_remaining - 1, visited);
    }

    count
}

fn parse(input: &str) -> Grid {
    let mut start: Point = Point { x: 0, y: 0 };
    let mut tiles = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Tile::Plot),
                '#' => row.push(Tile::Rock),
                'S' => {
                    row.push(Tile::Plot);
                    start = Point { x, y };
                }
                _ => panic!("Invalid character in input"),
            }
        }
        tiles.push(row);
    }
    let width = tiles[0].len();
    let height = tiles.len();
    Grid {
        tiles,
        width,
        height,
        start,
    }
}

fn solve(input: &str, steps: usize) -> usize {
    let grid = parse(input);
    let mut visited: HashSet<(Point, usize)> = HashSet::new();
    step(grid.start, &grid, steps, &mut visited)
}

fn main() {
    println!("Hello, world!");
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", solve(input, 64));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let example = include_str!("../../example.txt");
        assert_eq!(solve(example, 6), 16);
    }
}
