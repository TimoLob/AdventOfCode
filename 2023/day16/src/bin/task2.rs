use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Laser {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Tile {
    Empty(bool),
    NEMirror,  // \
    NWMirror,  // /
    VSplitter, // |
    HSplitter, // -
}

impl Direction {
    fn bounce_off_mirror(&self, tile: &Tile) -> Direction {
        match (self, tile) {
            (Direction::North, Tile::NEMirror) => Direction::West,
            (Direction::North, Tile::NWMirror) => Direction::East,

            (Direction::South, Tile::NEMirror) => Direction::East,
            (Direction::South, Tile::NWMirror) => Direction::West,

            (Direction::East, Tile::NEMirror) => Direction::South,
            (Direction::East, Tile::NWMirror) => Direction::North,

            (Direction::West, Tile::NEMirror) => Direction::North,
            (Direction::West, Tile::NWMirror) => Direction::South,

            v => panic!("Bounce of mirror called with invalid tile {:?}", v),
        }
    }
}

impl Laser {
    fn step(&self, width: usize, height: usize) -> Option<Laser> {
        match self.direction {
            Direction::North if self.y > 0 => Some(Laser {
                x: self.x,
                y: self.y - 1,
                direction: self.direction,
            }),
            Direction::South if self.y < height as i32 - 1 => Some(Laser {
                x: self.x,
                y: self.y + 1,
                direction: self.direction,
            }),
            Direction::East if self.x < width as i32 - 1 => Some(Laser {
                x: self.x + 1,
                y: self.y,
                direction: self.direction,
            }),
            Direction::West if self.x > 0 => Some(Laser {
                x: self.x - 1,
                y: self.y,
                direction: self.direction,
            }),
            _ => None,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<(Tile, HashSet<Laser>)>> {
    let output = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => (Tile::Empty(false), HashSet::new()),
                    '/' => (Tile::NWMirror, HashSet::new()),
                    '\\' => (Tile::NEMirror, HashSet::new()),
                    '|' => (Tile::VSplitter, HashSet::new()),
                    '-' => (Tile::HSplitter, HashSet::new()),
                    v => panic!("Unexpected character {}", v),
                })
                .collect::<Vec<(Tile, HashSet<Laser>)>>()
        })
        .collect();
    output
}

fn raytrace(laser: Laser, grid: &mut Vec<Vec<(Tile, HashSet<Laser>)>>) {
    let width = grid[0].len();
    let height = grid.len();
    //print!("{:?}", laser);
    // print_grid(grid);
    //println!("------------------------------");
    if let Some(mut next_laser) = laser.step(width, height) {
        let (tile, set) = &mut grid[next_laser.y as usize][next_laser.x as usize];
        // println!("{:?} {:?}", next_laser, tile);
        if set.contains(&next_laser) {
            return;
        }
        set.insert(next_laser);
        match tile {
            Tile::Empty(_) => {
                grid[next_laser.y as usize][next_laser.x as usize].0 = Tile::Empty(true);
                raytrace(next_laser, grid);
            }
            Tile::NEMirror | Tile::NWMirror => {
                let new_direction = next_laser.direction.bounce_off_mirror(tile);
                next_laser.direction = new_direction;
                raytrace(next_laser, grid);
            }
            Tile::VSplitter => match next_laser.direction {
                Direction::North | Direction::South => {
                    raytrace(next_laser, grid);
                }
                Direction::East | Direction::West => {
                    let laser1 = Laser {
                        x: next_laser.x,
                        y: next_laser.y,
                        direction: Direction::North,
                    };
                    let laser2 = Laser {
                        x: next_laser.x,
                        y: next_laser.y,
                        direction: Direction::South,
                    };
                    raytrace(laser1, grid);
                    raytrace(laser2, grid);
                }
            },
            Tile::HSplitter => match next_laser.direction {
                Direction::East | Direction::West => {
                    raytrace(next_laser, grid);
                }
                Direction::North | Direction::South => {
                    let laser1 = Laser {
                        x: next_laser.x,
                        y: next_laser.y,
                        direction: Direction::East,
                    };
                    let laser2 = Laser {
                        x: next_laser.x,
                        y: next_laser.y,
                        direction: Direction::West,
                    };
                    raytrace(laser1, grid);
                    raytrace(laser2, grid);
                }
            },
        }
    }
}

fn print_grid(grid: &Vec<Vec<(Tile, HashSet<Laser>)>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let (tile, _) = &grid[y][x];
            match tile {
                Tile::Empty(false) => print!("."),
                Tile::Empty(true) => print!("#"),
                Tile::NEMirror => print!("\\"),
                Tile::NWMirror => print!("/"),
                Tile::VSplitter => print!("|"),
                Tile::HSplitter => print!("-"),
            };
        }
        println!();
    }
}

fn print_energized(grid: &Vec<Vec<(Tile, HashSet<Laser>)>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let (tile, set) = &grid[y][x];
            if !set.is_empty() {
                print!("#");
                continue;
            }
            match tile {
                Tile::Empty(false) => print!("."),
                Tile::Empty(true) => print!("#"),
                Tile::NEMirror => print!("\\"),
                Tile::NWMirror => print!("/"),
                Tile::VSplitter => print!("|"),
                Tile::HSplitter => print!("-"),
            };
        }
        println!();
    }
}

fn solve_one(input: &str, start: Laser) -> i32 {
    let mut grid = parse(input);

    raytrace(start, &mut grid);
    // print_energized(&grid);
    let total = grid
        .iter()
        .map(|line| {
            line.iter().fold(0, |acc, (tile, set)| match tile {
                Tile::Empty(true) => acc + 1,
                _ if !set.is_empty() => acc + 1,
                _ => acc,
            })
        })
        .sum();
    total
}

fn solve(input: &str) -> i32 {
    let grid = parse(input);
    let width = grid[0].len();
    let height = grid.len();
    let mut max = 0;
    for x in 0..width {
        let start = Laser {
            x: x as i32,
            y: -1,
            direction: Direction::South,
        };
        let result = solve_one(input, start);
        if result > max {
            max = result;
        }
    }

    for x in 0..width {
        let start = Laser {
            x: x as i32,
            y: height as i32,
            direction: Direction::North,
        };
        let result = solve_one(input, start);
        if result > max {
            max = result;
            println!("New max {}", max);
        }
    }

    for y in 0..height {
        let start = Laser {
            x: -1,
            y: y as i32,
            direction: Direction::East,
        };
        let result = solve_one(input, start);
        if result > max {
            max = result;
        }

        let start = Laser {
            x: width as i32,
            y: y as i32,
            direction: Direction::West,
        };
        let result = solve_one(input, start);
        if result > max {
            max = result;
        }
    }

    max
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let example = include_str!("../../example.txt");
        assert_eq!(
            solve_one(
                example,
                Laser {
                    x: -1,
                    y: 0,
                    direction: Direction::East,
                },
            ),
            46
        );
    }
}
