#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    RoundRock,
    CubeRock,
}
#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            cells: vec![Cell::Empty; width * height],
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> &Cell {
        self.cells
            .get(y * self.width + x)
            .expect("Valid X,y in get")
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        self.cells
            .get_mut(y * self.width + x)
            .expect("Valid xy in get mut")
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let c = self.get_mut(x, y);
        *c = cell;
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Cell::Empty => print!("."),
                    Cell::RoundRock => print!("O"),
                    Cell::CubeRock => print!("#"),
                }
            }
            println!();
        }
    }

    fn from_str(s: &str) -> Grid {
        let mut width = 0;
        let mut height = 0;
        let mut cells = Vec::new();
        for line in s.lines() {
            width = line.len();
            height += 1;
            for c in line.chars() {
                cells.push(match c {
                    '.' => Cell::Empty,
                    'O' => Cell::RoundRock,
                    '#' => Cell::CubeRock,
                    _ => panic!("Invalid character"),
                });
            }
        }
        Grid {
            cells,
            width,
            height,
        }
    }
}

fn tilt_north(mut grid: Grid) -> Grid {
    // Tilt North
    // All Round Rocks will roll as far up as possible
    for y in 1..grid.height {
        for x in 0..grid.width {
            if *grid.get(x, y) == Cell::RoundRock {
                grid.set(x, y, Cell::Empty);
                let mut last_y = y;
                for yt in (0..y).rev() {
                    if *grid.get(x, yt) != Cell::Empty {
                        break;
                    }
                    last_y = yt;
                }
                grid.set(x, last_y, Cell::RoundRock);
            }
        }
    }
    grid
}

fn solve(mut grid: Grid) -> usize {
    grid = tilt_north(grid);
    grid.print();

    let mut total = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if *grid.get(x, y) == Cell::RoundRock {
                total += grid.height - y;
            }
        }
    }
    total
}

fn main() {
    let input = include_str!("../../input.txt");
    let grid = Grid::from_str(input);
    grid.print();
    let result = solve(grid);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = include_str!("../../example.txt");
        let grid = Grid::from_str(input);
        grid.print();
        println!();
        let result = solve(grid);
        assert_eq!(result, 136);
    }
}
