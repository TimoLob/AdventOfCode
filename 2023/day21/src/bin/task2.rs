use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

struct Grid<T> {
    tiles: Vec<Vec<T>>,
    width: usize,
    height: usize,
    start: Point,
}
impl<T> Grid<T> {
    fn get(&self, point: &Point) -> &T {
        &self.tiles[point.y.rem_euclid(self.height as i64) as usize]
            [point.x.rem_euclid(self.width as i64) as usize]
    }
}

fn step(
    pos: Point,
    grid: &Grid<Tile>,
    steps_remaining: usize,
    visited: &mut HashSet<(Point, usize)>,
    solved: &mut HashMap<(Point, usize), usize>,
) -> usize {
    let projected_point = Point {
        x: pos.x.rem_euclid(grid.width as i64),
        y: pos.y.rem_euclid(grid.height as i64),
    };
    if let Some(&count) = solved.get(&(projected_point, steps_remaining)) {
        return count;
    }

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
    if *grid.get(&new_pos) == Tile::Plot {
        count += step(new_pos, grid, steps_remaining - 1, visited, solved);
    }
    let mut new_pos = pos;
    new_pos.x -= 1;
    if *grid.get(&new_pos) == Tile::Plot {
        count += step(new_pos, grid, steps_remaining - 1, visited, solved);
    }
    let mut new_pos = pos;
    new_pos.y += 1;
    if *grid.get(&new_pos) == Tile::Plot {
        count += step(new_pos, grid, steps_remaining - 1, visited, solved);
    }
    let mut new_pos = pos;
    new_pos.y -= 1;
    if *grid.get(&new_pos) == Tile::Plot {
        count += step(new_pos, grid, steps_remaining - 1, visited, solved);
    }

    solved.insert((projected_point, steps_remaining), count);
    // println!("{:?}", solved);
    count
}

fn parse(input: &str) -> Grid<Tile> {
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
                    start = Point {
                        x: x as i64,
                        y: y as i64,
                    };
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
    let mut solved: HashMap<(Point, usize), usize> = HashMap::new();
    step(grid.start, &grid, steps, &mut visited, &mut solved)
}

fn main() {
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
    #[test]
    fn example2() {
        let example = include_str!("../../example.txt");
        assert_eq!(solve(example, 10), 50);
    }
    #[test]
    fn example3() {
        let example = include_str!("../../example.txt");
        assert_eq!(solve(example, 50), 1594);
    }
    #[test]
    fn example5() {
        let example = include_str!("../../example.txt");
        assert_eq!(solve(example, 5000), 16733044);
    }
}
