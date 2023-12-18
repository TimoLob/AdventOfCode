use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    dist: i64,
    x: u32,
    y: u32,
    dir: Direction,
    same_steps: u8,
    prev: (u32, u32),
}

fn solve(input: &str) -> i64 {
    let grid = parse(input);
    return dijkstra(grid);
}

fn dijkstra(grid: Vec<Vec<u32>>) -> i64 {
    let mut heap = BinaryHeap::new();
    let mut visited: HashSet<(u32, u32, Direction, u8)> = HashSet::new();
    let mut dist: HashMap<(u32, u32, Direction, u8), i64> = HashMap::new();

    let width = grid[0].len();
    let height = grid.len();
    heap.push(Node {
        dist: 0,
        x: 0,
        y: 0,
        dir: Direction::South,
        same_steps: 0,
        prev: (0, 0),
    });
    heap.push(Node {
        dist: 0,
        x: 0,
        y: 0,
        dir: Direction::East,
        same_steps: 0,
        prev: (0, 0),
    });
    while let Some(node) = heap.pop() {
        if visited.contains(&(node.x, node.y, node.dir, node.same_steps)) {
            continue;
        }
        visited.insert((node.x, node.y, node.dir, node.same_steps));
        if node.x as usize == width - 1 && node.y as usize == height - 1 && node.same_steps >= 4 {
            println!("TARGET : {:?}", node);
            return -node.dist;
        }
        let mut invalid_directions = vec![match node.dir {
            // No Backtracking
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }];
        // println!("node : {:?} Dir: ", node);
        if node.same_steps == 10 {
            invalid_directions.push(node.dir);
        }
        let mut valid_directions = vec![];
        if node.y > 3 && !invalid_directions.contains(&Direction::North) {
            valid_directions.push(Direction::North);
        }
        if node.x > 3 && !invalid_directions.contains(&Direction::West) {
            valid_directions.push(Direction::West);
        }
        if (node.y as usize) < height - 4 && !invalid_directions.contains(&Direction::South) {
            valid_directions.push(Direction::South);
        }
        if (node.x as usize) < width - 4 && !invalid_directions.contains(&Direction::East) {
            valid_directions.push(Direction::East);
        }
        if node.same_steps < 4 {
            valid_directions.clear();
            valid_directions.push(node.dir);
        }
        // println!("Valid Directions: {:?}", valid_directions);
        for dir in valid_directions.iter() {
            let nx;
            let ny;
            match dir {
                Direction::North => {
                    nx = node.x;
                    ny = node.y - 1;
                }
                Direction::South => {
                    nx = node.x;
                    ny = node.y + 1;
                }
                Direction::East => {
                    nx = node.x + 1;
                    ny = node.y;
                }
                Direction::West => {
                    nx = node.x - 1;
                    ny = node.y;
                }
            }
            let same_steps = if *dir == node.dir {
                node.same_steps + 1
            } else {
                1
            };
            // println!("Direction {:?} NX : {} NY : {}", dir, nx, ny);
            let cost = grid[ny as usize][nx as usize];
            let alt = -node.dist + cost as i64;
            if !dist.contains_key(&(nx, ny, *dir, same_steps))
                || *dist
                    .get(&(node.x, node.y, node.dir, node.same_steps))
                    .unwrap()
                    > alt
            {
                dist.insert((nx, ny, *dir, same_steps), alt);
                heap.push(Node {
                    dist: -alt,
                    x: nx,
                    y: ny,
                    dir: *dir,
                    same_steps,
                    prev: (node.x, node.y),
                });
            }
        }
    }

    0
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut nodes = vec![];
    for line in input.lines() {
        let mut row_nodes = vec![];
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                row_nodes.push(digit);
                print!("{}", digit);
            } else {
                panic!("Unexpected character {}", c);
            }
        }
        nodes.push(row_nodes);
        println!();
    }
    nodes
}

fn main() {
    let input = include_str!("../../input.txt");

    let result = solve(input);
    println!("Result : {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = include_str!("../../example.txt");
        let result = solve(input);
        assert_eq!(result, 94);
    }
}
