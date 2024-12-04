fn check_grid(grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let hlen = grid.len();
    let vlen = grid[0].len();
    let mut total = 0;
    // Check to the Right
    if i + 3 < hlen {
        if grid[i + 1][j] == 'M' && grid[i + 2][j] == 'A' && grid[i + 3][j] == 'S' {
            total += 1;
        }
        if j + 3 < vlen {
            // Bottom Right
            if grid[i + 1][j + 1] == 'M' && grid[i + 2][j + 2] == 'A' && grid[i + 3][j + 3] == 'S' {
                total += 1
            }
        }
        // Top right
        if j.checked_sub(3).is_some() {
            if grid[i + 1][j - 1] == 'M' && grid[i + 2][j - 2] == 'A' && grid[i + 3][j - 3] == 'S' {
                total += 1;
            }
        }
    }
    // Left
    if i.checked_sub(3).is_some() {
        if grid[i - 1][j] == 'M' && grid[i - 2][j] == 'A' && grid[i - 3][j] == 'S' {
            total += 1;
        }
        // Bottom Left
        if j + 3 < vlen {
            if grid[i - 1][j + 1] == 'M' && grid[i - 2][j + 2] == 'A' && grid[i - 3][j + 3] == 'S' {
                total += 1
            }
        }
        // Top left
        if j.checked_sub(3).is_some() {
            if grid[i - 1][j - 1] == 'M' && grid[i - 2][j - 2] == 'A' && grid[i - 3][j - 3] == 'S' {
                total += 1;
            }
        }
    }
    // Down
    if j.checked_sub(3).is_some() {
        if grid[i][j - 1] == 'M' && grid[i][j - 2] == 'A' && grid[i][j - 3] == 'S' {
            total += 1;
        }
    }
    if j + 3 < vlen {
        if grid[i][j + 1] == 'M' && grid[i][j + 2] == 'A' && grid[i][j + 3] == 'S' {
            total += 1;
        }
    }
    total
}

fn x_mas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let hlen = grid.len();
    let vlen = grid[0].len();
    if i.checked_sub(1).is_none() || i + 1 >= hlen || j.checked_sub(1).is_none() || j + 1 >= vlen {
        return 0;
    }

    let top_left_to_bottom_right = (grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S')
        || (grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M');
    let bottom_left_to_top_right = (grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S')
        || (grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M');
    if top_left_to_bottom_right && bottom_left_to_top_right {
        1
    } else {
        0
    }
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                total += check_grid(&grid, i, j);
            }
        }
    }
    total.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'A' {
                total += x_mas(&grid, i, j);
            }
        }
    }
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
        assert_eq!(result, "18"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "9"); // Replace with the actual expected result
    }
}
