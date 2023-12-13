#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Ash,  // "."
    Rock, // "#"
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.cells[y][x])
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y][x] = cell;
    }

    fn from_string(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    '.' => Cell::Ash,
                    '#' => Cell::Rock,
                    _ => panic!("Invalid cell"),
                });
            }
            cells.push(row);
            width = line.len();
            height += 1;
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn print(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Ash => print!("."),
                    Cell::Rock => print!("#"),
                }
            }
            println!();
        }
    }
}

/*
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
*/
fn parse(input: &str) -> Vec<Grid> {
    // Multiple Grids separated by empty lines
    let mut grids = Vec::new();
    // Split input string at empty lines
    let grid_strings = input.split("\n\n");
    for grid_string in grid_strings {
        grids.push(Grid::from_string(grid_string));
    }
    grids
}

fn find_horizontal_symmetry(grid: &Grid) -> Option<usize> {
    // Find horizontal symmetry axis
    for axis in 1..grid.width {
        // Check if x is a possible symmetry axis
        let dx = (grid.width - axis).min(axis);
        let mut differences = 0;
        // println!("Axis: {}, dx: {}", axis, dx);
        let possible = (0..grid.height).all(|y| {
            //println!("y: {}", y);
            let mut left = axis - dx;
            let mut right = axis + dx - 1;

            while left < right {
                let left_cell = *grid.get(left, y).unwrap();
                let right_cell = *grid.get(right, y).unwrap();
                //println!("left: {:?} ({}), right: {:?} ({})",left_cell, left, right_cell, right);
                if left_cell != right_cell {
                    differences += 1;
                    if differences > 1 {
                        return false;
                    }
                }
                left += 1;
                right -= 1;
            }
            true
        });
        if possible && differences == 1 {
            return Some(axis);
        }
    }
    None
}

fn find_vertical_symmetry(grid: &Grid) -> Option<usize> {
    // Find vertical symmetry axis
    for axis in 1..grid.height {
        // Check if x is a possible symmetry axis
        let dy = (grid.height - axis).min(axis);
        let mut differences = 0;
        //println!("Axis: {}, dx: {}", axis, dy);
        let possible = (0..grid.width).all(|x| {
            //println!("x: {}", x);
            let mut left = axis - dy;
            let mut right = axis + dy - 1;

            while left < right {
                let left_cell = *grid.get(x, left).unwrap();
                let right_cell = *grid.get(x, right).unwrap();
                //println!("left: {:?} ({}), right: {:?} ({})",left_cell, left, right_cell, right);
                if left_cell != right_cell {
                    differences += 1;
                    if differences > 1 {
                        return false;
                    }
                }
                left += 1;
                right -= 1;
            }
            true
        });
        if possible && differences == 1 {
            return Some(axis);
        }
    }
    None
}

fn main() {
    let input = include_str!("../../input.txt");
    let grids = parse(input);
    let mut total = 0usize;
    for grid in grids {
        grid.print();
        let axis = find_horizontal_symmetry(&grid);
        total += axis.unwrap_or(0);
        println!("Horizontal symmetry axis: {:?}", axis);
        let axis = find_vertical_symmetry(&grid);
        total += axis.unwrap_or(0) * 100;
        println!("Vertical symmetry axis: {:?}", axis);
        println!("---------------");
    }
    println!("Total: {}", total);
}
