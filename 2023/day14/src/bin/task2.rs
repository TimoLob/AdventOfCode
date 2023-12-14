use hashbrown::HashMap;

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
    fn hash(&self) -> u32 {
        let mut hash = 1u32;
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Cell::Empty => hash = hash.wrapping_mul(31).wrapping_add(b'.' as u32),
                    Cell::RoundRock => hash = hash.wrapping_mul(31).wrapping_add(b'O' as u32),
                    Cell::CubeRock => hash = hash.wrapping_mul(31).wrapping_add(b'#' as u32),
                }
            }
        }
        hash
    }

    fn from_str(s: &str) -> Grid {
        let mut width = 0;
        let mut height = 0;
        let mut cells = Vec::new();
        for line in s.lines() {
            width = line.len();
            height += 1;
            for c in line.chars() {
                let cell = match c {
                    '.' => Cell::Empty,
                    'O' => Cell::RoundRock,
                    '#' => Cell::CubeRock,
                    _ => panic!("Invalid character"),
                };

                cells.push(cell);
            }
        }
        Grid {
            cells,
            width,
            height,
        }
    }

    fn tilt_west(&mut self) {
        // Tilt West
        // All Round Rocks will roll as far left as possible
        for x in 1..self.width {
            for y in 0..self.height {
                if *self.get(x, y) == Cell::RoundRock {
                    self.set(x, y, Cell::Empty);
                    let mut last_x = x;
                    for xt in (0..x).rev() {
                        if *self.get(xt, y) != Cell::Empty {
                            break;
                        }
                        last_x = xt;
                    }
                    self.set(last_x, y, Cell::RoundRock);
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        // Tilt East
        // All Round Rocks will roll as far right as possible
        for x in (0..self.width - 1).rev() {
            for y in 0..self.height {
                if *self.get(x, y) == Cell::RoundRock {
                    self.set(x, y, Cell::Empty);
                    let mut last_x = x;
                    for xt in x + 1..self.width {
                        if *self.get(xt, y) != Cell::Empty {
                            break;
                        }
                        last_x = xt;
                    }
                    self.set(last_x, y, Cell::RoundRock);
                }
            }
        }
    }

    fn tilt_north(&mut self) {
        // Tilt North
        // All Round Rocks will roll as far up as possible
        for y in 1..self.height {
            for x in 0..self.width {
                if *self.get(x, y) == Cell::RoundRock {
                    self.set(x, y, Cell::Empty);
                    let mut last_y = y;
                    for yt in (0..y).rev() {
                        if *self.get(x, yt) != Cell::Empty {
                            break;
                        }
                        last_y = yt;
                    }
                    self.set(x, last_y, Cell::RoundRock);
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        // Tilt South
        // All Round Rocks will roll as far down as possible
        for y in (0..self.height - 1).rev() {
            for x in 0..self.width {
                if *self.get(x, y) == Cell::RoundRock {
                    self.set(x, y, Cell::Empty);
                    let mut last_y = y;
                    for yt in y + 1..self.height {
                        if *self.get(x, yt) != Cell::Empty {
                            break;
                        }
                        last_y = yt;
                    }
                    self.set(x, last_y, Cell::RoundRock);
                }
            }
        }
    }
}

fn cycle(grid: Grid, n: usize) -> Grid {
    let mut grid = grid;
    let mut loop_found = false;
    let mut seen_states = HashMap::new();

    let mut i = 0usize;
    seen_states.insert(grid.hash(), i);
    while i < n {
        if i % 100000 == 0 {
            println!("Cycle: {} %", i as f32 * 100.0 / n as f32);
        }
        grid.tilt_north();
        grid.tilt_west();
        grid.tilt_south();
        grid.tilt_east();
        i += 1;
        let hash = grid.hash();
        if !loop_found {
            if let Some(prev_i) = seen_states.get(&hash) {
                let cycle_len = i - prev_i;
                let repeat = (n - i) / cycle_len;
                println!(
                    "Loop found: at {} len {}, starts at {} , repeats: {}",
                    i, cycle_len, prev_i, repeat
                );
                i += repeat * cycle_len;
                loop_found = true;
            }
        }

        seen_states.insert(grid.hash(), i);
    }
    grid
}

fn solve(mut grid: Grid) -> usize {
    grid = cycle(grid, 1_000_000_000);
    //grid.print();

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
    //grid.print();
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
        assert_eq!(result, 64);
    }
}
