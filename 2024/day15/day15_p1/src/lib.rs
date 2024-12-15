use glam::IVec2;

#[derive(Debug, Clone, Copy)]
enum Tile {
    EMPTY,
    BOX,
    WALL,
}

struct Grid<T: Copy> {
    array: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Grid<T> {
    fn new(width: usize, height: usize, default: T) -> Self {
        let mut v: Vec<T> = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            v.push(default);
        }
        return Grid {
            array: v,
            width,
            height,
        };
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        } else {
            return self.array.get(y * self.width + x);
        }
    }

    fn set(&mut self, x: usize, y: usize, data: T) -> Result<(), String> {
        if let Some(element) = self.array.get_mut(self.width * y + x) {
            *element = data;
            return Ok(());
        }
        Err(format!("Index {} {} is out of bounds.", x, y))
    }
}

#[allow(dead_code)]
impl Grid<Tile> {
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get(x, y).unwrap();
                let c = match tile {
                    Tile::EMPTY => '.',
                    Tile::BOX => 'O',
                    Tile::WALL => '#',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}
#[derive(Debug)]
enum Instruction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Instruction {
    fn get_dir(&self) -> IVec2 {
        match self {
            Instruction::NORTH => IVec2::NEG_Y,
            Instruction::SOUTH => IVec2::Y,
            Instruction::EAST => IVec2::X,
            Instruction::WEST => IVec2::NEG_X,
        }
    }
}
#[derive(Debug)]
struct Robot {
    pos: IVec2,
}

fn parse_grid(input: &str) -> (Robot, Grid<Tile>) {
    let height = input.split('\n').count();
    let width = input.split('\n').next().unwrap().chars().count();
    let mut grid = Grid::new(width, height, Tile::EMPTY);
    let mut robot = Robot {
        pos: IVec2 { x: 0, y: 0 },
    };
    input.split('\n').enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '#' => _ = grid.set(x, y, Tile::WALL),
            'O' => _ = grid.set(x, y, Tile::BOX),
            '@' => {
                robot.pos = IVec2 {
                    x: x as i32,
                    y: y as i32,
                }
            }
            '.' => {}
            _ => unreachable!(),
        });
    });
    (robot, grid)
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => Instruction::NORTH,
            'v' => Instruction::SOUTH,
            '<' => Instruction::WEST,
            '>' => Instruction::EAST,
            _ => unreachable!(),
        })
        .collect()
}
fn parse(input: &str) -> (Robot, Grid<Tile>, Vec<Instruction>) {
    let (grid_input, instruction_input) = input.split_once("\n\n").unwrap();
    let (robot, grid) = parse_grid(grid_input);
    let instructions = parse_instructions(instruction_input);
    (robot, grid, instructions)
}

fn move_robot(robot: &mut Robot, grid: &mut Grid<Tile>, dir: &Instruction) {
    let dir = dir.get_dir();
    let new_pos = robot.pos + dir;
    if let Some(new_tile) = grid.get(new_pos.x as usize, new_pos.y as usize) {
        match new_tile {
            Tile::EMPTY => {
                robot.pos = new_pos;
                return;
            }
            Tile::BOX => {
                if move_box(new_pos, dir, grid).is_ok() {
                    robot.pos = new_pos;
                }
                return;
            }
            Tile::WALL => {
                return;
            }
        };
    }
    return;
}

fn move_box(pos: IVec2, dir: IVec2, grid: &mut Grid<Tile>) -> Result<(), ()> {
    let new_pos = pos + dir;
    if let Some(new_tile) = grid.get(new_pos.x as usize, new_pos.y as usize) {
        match new_tile {
            Tile::EMPTY => {
                _ = grid.set(pos.x as usize, pos.y as usize, Tile::EMPTY);
                _ = grid.set(new_pos.x as usize, new_pos.y as usize, Tile::BOX);
                return Ok(());
            }
            Tile::BOX => {
                if move_box(new_pos, dir, grid).is_ok() {
                    _ = grid.set(pos.x as usize, pos.y as usize, Tile::EMPTY);
                    _ = grid.set(new_pos.x as usize, new_pos.y as usize, Tile::BOX);
                    return Ok(());
                } else {
                    return Err(());
                }
            }
            Tile::WALL => return Err(()),
        }
    }
    Err(())
}

fn get_gps_coords(grid: &Grid<Tile>) -> usize {
    let mut total = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            let tile = grid.get(x, y).unwrap();
            match tile {
                Tile::EMPTY => {}
                Tile::BOX => total += 100 * y + x,
                Tile::WALL => {}
            };
        }
    }
    total
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let (mut robot, mut grid, instructions) = parse(input);
    instructions.iter().for_each(|instruction| {
        move_robot(&mut robot, &mut grid, instruction);
    });
    let total = get_gps_coords(&grid);
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "10092"); // Replace with the actual expected result
    }

    #[test]
    fn test_example_small() {
        let input = fs::read_to_string("example_small.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "2028"); // Replace with the actual expected result
    }
}
