use hex_color::HexColor;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Ground,
    Trench(HexColor),
    Inside,
    Outside,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    color: HexColor,
    steps: i32,
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            match grid[x][y] {
                Cell::Ground => print!("."),
                Cell::Trench(_) => print!("#"),
                Cell::Inside => print!("I"),
                Cell::Outside => print!("O"),
            }
        }
        println!();
    }
}

fn floodfill(grid: &mut Vec<Vec<Cell>>) {
    let mut queue = Vec::new();
    queue.push((0, 0));
    while let Some((x, y)) = queue.pop() {
        if grid[x][y] != Cell::Ground {
            continue;
        }
        grid[x][y] = Cell::Outside;
        if x > 0 {
            queue.push((x - 1, y));
        }
        if y > 0 {
            queue.push((x, y - 1));
        }
        if x < grid.len() - 1 {
            queue.push((x + 1, y));
        }
        if y < grid[0].len() - 1 {
            queue.push((x, y + 1));
        }
    }
}

fn solve(input: &str) -> i32 {
    // Create a 500x500 grid of Ground Cells
    let mut grid = vec![vec![Cell::Ground; 500]; 500];

    let mut x = grid.len() / 2;
    let mut y = grid[0].len() / 2;

    let instructions = parse(input);
    instructions.iter().for_each(|instruction| {
        for _ in 0..instruction.steps {
            match instruction.direction {
                Direction::Up => y -= 1,
                Direction::Down => y += 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            };
            if x > grid.len() {
                panic!("X out of bounds {}", x);
            }
            if y > grid[0].len() {
                panic!("y out of bounds {}", y);
            }
            grid[x][y] = Cell::Trench(instruction.color);
        }
    });
    floodfill(&mut grid);

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == Cell::Ground {
                grid[x][y] = Cell::Inside;
            }
        }
    }

    print_grid(&grid);
    // Count the number of cells that are inside and trenches
    let mut inside = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            match grid[x][y] {
                Cell::Inside | Cell::Trench(_) => inside += 1,
                _ => (),
            }
        }
    }

    inside
}

fn parse(input: &str) -> Vec<Instruction> {
    let instructions = input
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let d = match parts[0] {
                "R" => Direction::Right,
                "D" => Direction::Down,
                "U" => Direction::Up,
                "L" => Direction::Left,
                v => panic!("Unexpected part 1 of input {}", v),
            };
            let steps = parts[1]
                .parse::<i32>()
                .expect("Expect part 2 to parse to int");
            let color = HexColor::parse(&parts[2].replace(['(', ')'], "")).expect("Expect Hex");
            Instruction {
                direction: d,
                color,
                steps,
            }
        })
        .collect();
    instructions
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result: {}", result);
}
