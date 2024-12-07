use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Tile {
    EMPTY,
    WALL,
}
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
}

impl Guard {
    fn step(&mut self) -> Position {
        match self.direction {
            Direction::NORTH => Position {
                x: self.pos.x,
                y: self.pos.y - 1,
            },
            Direction::EAST => Position {
                x: self.pos.x + 1,
                y: self.pos.y,
            },
            Direction::SOUTH => Position {
                x: self.pos.x,
                y: self.pos.y + 1,
            },
            Direction::WEST => Position {
                x: self.pos.x - 1,
                y: self.pos.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Guard {
    pos: Position,
    direction: Direction,
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let mut map: HashMap<Position, Tile> = HashMap::new();
    let mut guard = Guard {
        pos: Position { x: 0, y: 0 },
        direction: Direction::NORTH,
    };
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };
            match c {
                '.' => _ = map.insert(pos, Tile::EMPTY),
                '#' => _ = map.insert(pos, Tile::WALL),
                '^' => {
                    guard.pos = pos;
                    _ = map.insert(pos, Tile::EMPTY);
                }
                _ => unreachable!(),
            }
        })
    });

    let mut visited: HashSet<Position> = HashSet::new();
    while let Some(_) = map.get(&guard.pos) {
        visited.insert(guard.pos);
        let next_pos = guard.step();
        if let Some(tile) = map.get(&next_pos) {
            match tile {
                Tile::EMPTY => guard.pos = next_pos,
                Tile::WALL => guard.direction = guard.direction.turn(),
            };
        } else {
            guard.pos = next_pos;
        }
    }

    visited.len().to_string()
}

#[derive(PartialEq, Eq, Debug)]
enum Result {
    LOOP,
    NoLoop,
}

fn walk_with_loop_detection(mut guard: Guard, map: &HashMap<Position, Tile>) -> Result {
    let mut visited: HashSet<Guard> = HashSet::new();
    while let Some(_) = map.get(&guard.pos) {
        if visited.contains(&guard) {
            return Result::LOOP;
        }
        visited.insert(guard);
        let next_pos = guard.step();
        if let Some(tile) = map.get(&next_pos) {
            match tile {
                Tile::EMPTY => guard.pos = next_pos,
                Tile::WALL => guard.direction = guard.direction.turn(),
            };
        } else {
            guard.pos = next_pos;
        }
    }
    Result::NoLoop
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let mut map: HashMap<Position, Tile> = HashMap::new();
    let mut guard = Guard {
        pos: Position { x: 0, y: 0 },
        direction: Direction::NORTH,
    };
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };
            match c {
                '.' => _ = map.insert(pos, Tile::EMPTY),
                '#' => _ = map.insert(pos, Tile::WALL),
                '^' => {
                    guard.pos = pos;
                    _ = map.insert(pos, Tile::EMPTY);
                }
                _ => unreachable!(),
            }
        })
    });
    let start_position = guard.pos;
    // Part 1 to get all positions where we will place an obstacle
    let mut visited: HashSet<Position> = HashSet::new();
    while let Some(_) = map.get(&guard.pos) {
        visited.insert(guard.pos);
        let next_pos = guard.step();
        if let Some(tile) = map.get(&next_pos) {
            match tile {
                Tile::EMPTY => guard.pos = next_pos,
                Tile::WALL => guard.direction = guard.direction.turn(),
            };
        } else {
            guard.pos = next_pos;
        }
    }

    visited.remove(&start_position);
    let mut number_of_loops = 0;
    for obstacle_position in visited.iter() {
        map.insert(*obstacle_position, Tile::WALL);
        let guard = Guard {
            pos: start_position,
            direction: Direction::NORTH,
        };
        if Result::LOOP == walk_with_loop_detection(guard, &map) {
            number_of_loops += 1;
        }
        map.insert(*obstacle_position, Tile::EMPTY);
    }
    number_of_loops.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "41"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "6"); // Replace with the actual expected result
    }
}
