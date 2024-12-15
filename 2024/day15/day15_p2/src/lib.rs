use std::collections::VecDeque;

use glam::IVec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    EMPTY,
    BoxLeft,
    BoxRight,
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
    fn print(&self, robot: &Robot) {
        for y in 0..self.height {
            for x in 0..self.width {
                if robot.pos.x as usize == x && robot.pos.y as usize == y {
                    print!("@");
                    continue;
                }
                let tile = self.get(x, y).unwrap();
                let c = match tile {
                    Tile::EMPTY => '.',
                    Tile::BoxLeft => '[',
                    Tile::BoxRight => ']',
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
    let width = input.split('\n').next().unwrap().chars().count() * 2;
    let mut grid = Grid::new(width, height, Tile::EMPTY);
    let mut robot = Robot {
        pos: IVec2 { x: 0, y: 0 },
    };
    input.split('\n').enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '#' => {
                _ = grid.set(x * 2, y, Tile::WALL);
                _ = grid.set(x * 2 + 1, y, Tile::WALL);
            }
            'O' => {
                _ = grid.set(x * 2, y, Tile::BoxLeft);
                _ = grid.set(x * 2 + 1, y, Tile::BoxRight);
            }
            '@' => {
                robot.pos = IVec2 {
                    x: (x * 2) as i32,
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
    let dir_vec = dir.get_dir();
    let new_pos = robot.pos + dir_vec;
    if let Some(new_tile) = grid.get(new_pos.x as usize, new_pos.y as usize) {
        match new_tile {
            Tile::EMPTY => {
                robot.pos = new_pos;
                return;
            }
            Tile::BoxLeft | Tile::BoxRight => {
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

fn move_box_up_down(start_pos: IVec2, grid: &mut Grid<Tile>, dir: IVec2) -> Result<(), ()> {
    let mut boxes_queue: VecDeque<IVec2> = VecDeque::new();
    let mut boxes: Vec<IVec2> = Vec::new();
    let tile = grid
        .get(start_pos.x as usize, start_pos.y as usize)
        .unwrap();

    boxes_queue.push_back(start_pos);
    if *tile == Tile::BoxRight {
        boxes_queue.push_back(start_pos + IVec2::NEG_X);
    } else if *tile == Tile::BoxLeft {
        boxes_queue.push_back(start_pos + IVec2::X);
    } else {
        unreachable!()
    }
    while !boxes_queue.is_empty() {
        let pos = boxes_queue.pop_front().unwrap();
        boxes.push(pos);
        let new_pos = pos + dir;
        let tile = grid.get(new_pos.x as usize, new_pos.y as usize).unwrap();
        match tile {
            Tile::EMPTY => {}
            Tile::BoxLeft => {
                if !boxes_queue.contains(&new_pos) {
                    boxes_queue.push_back(new_pos);
                }

                if !boxes_queue.contains(&(new_pos + IVec2::X)) {
                    boxes_queue.push_back(new_pos + IVec2::X);
                }
            }
            Tile::BoxRight => {
                if !boxes_queue.contains(&new_pos) {
                    boxes_queue.push_back(new_pos);
                }

                if !boxes_queue.contains(&(new_pos + IVec2::NEG_X)) {
                    boxes_queue.push_back(new_pos + IVec2::NEG_X);
                }
            }
            Tile::WALL => return Err(()),
        }
    }
    boxes.iter().rev().for_each(|pos| {
        let new_pos = pos + dir;
        _ = grid.set(
            new_pos.x as usize,
            new_pos.y as usize,
            *grid.get(pos.x as usize, pos.y as usize).unwrap(),
        );
        let down = pos - dir;
        if !boxes.contains(&down) {
            _ = grid.set(pos.x as usize, pos.y as usize, Tile::EMPTY);
        }
    });
    _ = grid.set(start_pos.x as usize, start_pos.y as usize, Tile::EMPTY);
    Ok(())
}

fn move_box(pos: IVec2, dir: &Instruction, grid: &mut Grid<Tile>) -> Result<(), ()> {
    match dir {
        Instruction::NORTH => return move_box_up_down(pos, grid, IVec2::NEG_Y),
        Instruction::SOUTH => return move_box_up_down(pos, grid, IVec2::Y),
        Instruction::EAST | Instruction::WEST => {
            let mut current_pos = pos;
            let can_move = loop {
                let next_tile = grid
                    .get(current_pos.x as usize, current_pos.y as usize)
                    .unwrap();

                match next_tile {
                    Tile::EMPTY => break true,
                    Tile::WALL => break false,
                    Tile::BoxRight => current_pos += dir.get_dir(),
                    Tile::BoxLeft => current_pos += dir.get_dir(),
                };
            };
            if !can_move {
                return Err(());
            }
            while current_pos != pos {
                let next_pos = current_pos - dir.get_dir();
                _ = grid.set(
                    current_pos.x as usize,
                    current_pos.y as usize,
                    *grid.get(next_pos.x as usize, next_pos.y as usize).unwrap(),
                );
                current_pos = next_pos;
            }

            _ = grid.set(pos.x as usize, pos.y as usize, Tile::EMPTY);
            return Ok(());
        }
    };
}

fn get_gps_coords(grid: &Grid<Tile>) -> usize {
    let mut total = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            let tile = grid.get(x, y).unwrap();
            match tile {
                Tile::BoxLeft => total += 100 * y + x,
                _ => {}
            };
        }
    }
    total
}

pub fn part2(input: &str) -> String {
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
        let result = part2(&input);
        assert_eq!(result, "9021"); // Replace with the actual expected result
    }
}
