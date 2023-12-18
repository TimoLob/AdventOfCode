#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Point {
    x: i64,
    y: i64,
}
struct Instruction {
    direction: Direction,
    steps: i64,
}

fn solve(input: &str) -> i64 {
    let instructions = parse(input);
    let mut x = 0;
    let mut y = 0;
    let mut points = Vec::new();
    println!("x: {}, y: {}", x, y);
    points.push(Point { x, y });
    instructions.iter().for_each(|instruction| {
        match instruction.direction {
            Direction::Up => y -= instruction.steps,
            Direction::Down => y += instruction.steps,
            Direction::Left => x -= instruction.steps,
            Direction::Right => x += instruction.steps,
        };
        points.push(Point { x, y });
        println!("x: {}, y: {}", x, y);
    });
    println!("Len: {}", points.len());

    // Shoelace formula to calculate area of polygon
    let mut area: i64 = 0;
    for i in 0..points.len() {
        area += (points[i].y + points[(i + 1) % points.len()].y)
            * (points[i].x - points[(i + 1) % points.len()].x);
    }

    println!("Area: {}", area);
    let i = area.abs() / 2;
    // Picks Theorem
    let b = instructions.iter().fold(0, |acc, p| acc + p.steps);
    println!("i: {}, b: {}", i, b);
    i + b / 2 + 1
}

fn parse(input: &str) -> Vec<Instruction> {
    let instructions = input
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();

            let color = &parts[2].replace(['(', ')'], "");
            let steps = &color[1..6];
            // Convert steps to int from Hex String
            let steps = i64::from_str_radix(steps, 16).unwrap();
            let dir = &color[6..7];
            let direction = match dir {
                "3" => Direction::Up,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "0" => Direction::Right,
                _ => panic!("Unknown direction {}", dir),
            };
            println!("Color : {} , steps: {}, dir: {:?}", color, steps, dir);

            Instruction { direction, steps }
        })
        .collect();
    instructions
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result: {}", result);
}
