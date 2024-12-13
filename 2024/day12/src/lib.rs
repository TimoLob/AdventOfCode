use std::collections::{HashSet, VecDeque};

use array2d::Array2D;

fn process_area(grid: &mut Array2D<char>, pos: (usize, usize)) -> usize {
    let crop = grid.get(pos.0, pos.1).unwrap();
    if *crop == ' ' {
        return 0;
    }
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![pos]);
    let mut area = 0;
    let mut perimeter = 0;
    let mut already_handled: HashSet<(usize, usize)> = HashSet::new();
    already_handled.insert(pos);
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        area += 1;
        let new_pos = (pos.0 - 1, pos.1);
        if let Some(_other_crop) = grid.get(new_pos.0, new_pos.1).filter(|&c| c == crop) {
            if !already_handled.contains(&new_pos) {
                queue.push_back(new_pos);
                already_handled.insert(new_pos);
            }
        } else {
            perimeter += 1;
        }
        let new_pos = (pos.0 + 1, pos.1);
        if let Some(_other_crop) = grid.get(new_pos.0, new_pos.1).filter(|&c| c == crop) {
            if !already_handled.contains(&new_pos) {
                queue.push_back(new_pos);
                already_handled.insert(new_pos);
            }
        } else {
            perimeter += 1;
        }
        let new_pos = (pos.0, pos.1 - 1);
        if let Some(_other_crop) = grid.get(new_pos.0, new_pos.1).filter(|&c| c == crop) {
            if !already_handled.contains(&new_pos) {
                queue.push_back(new_pos);
                already_handled.insert(new_pos);
            }
        } else {
            perimeter += 1;
        }
        let new_pos = (pos.0, pos.1 + 1);
        if let Some(_other_crop) = grid.get(new_pos.0, new_pos.1).filter(|&c| c == crop) {
            if !already_handled.contains(&new_pos) {
                queue.push_back(new_pos);
                already_handled.insert(new_pos);
            }
        } else {
            perimeter += 1;
        }
    }
    already_handled
        .iter()
        .for_each(|&pos| _ = grid.set(pos.0, pos.1, ' '));
    area * perimeter
}

// Hint : A region has the same amount of sides as corners.
//
//
//
fn process_area_part2(grid: &mut Array2D<char>, pos: (usize, usize)) -> usize {
    let crop = grid.get(pos.0, pos.1).unwrap();
    if *crop == ' ' {
        return 0;
    }
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![pos]);
    let mut area = 0;
    let mut perimeter = 0;
    let mut already_handled: HashSet<(usize, usize)> = HashSet::new();
    already_handled.insert(pos);
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        area += 1;
        let new_pos = (pos.0 - 1, pos.1);
        if let Some(_other_crop) = grid.get(new_pos.0, new_pos.1).filter(|&c| c == crop) {
            if !already_handled.contains(&new_pos) {
                queue.push_back(new_pos);
                already_handled.insert(new_pos);
            }
        } else {
            perimeter += 1;
        }
        let new_pos = (pos.0 + 1, pos.1);
        if let Some(_other_crop) = grid.get(new_pos.0, new_pos.1).filter(|&c| c == crop) {
            if !already_handled.contains(&new_pos) {
                queue.push_back(new_pos);
                already_handled.insert(new_pos);
            }
        } else {
            perimeter += 1;
        }
        let new_pos = (pos.0, pos.1 - 1);
        if let Some(_other_crop) = grid.get(new_pos.0, new_pos.1).filter(|&c| c == crop) {
            if !already_handled.contains(&new_pos) {
                queue.push_back(new_pos);
                already_handled.insert(new_pos);
            }
        } else {
            perimeter += 1;
        }
        let new_pos = (pos.0, pos.1 + 1);
        if let Some(_other_crop) = grid.get(new_pos.0, new_pos.1).filter(|&c| c == crop) {
            if !already_handled.contains(&new_pos) {
                queue.push_back(new_pos);
                already_handled.insert(new_pos);
            }
        } else {
            perimeter += 1;
        }
    }
    already_handled
        .iter()
        .for_each(|&pos| _ = grid.set(pos.0, pos.1, ' '));

    let mut corners = 0;

    area * perimeter
}

fn print_grid(array: &Array2D<char>) {
    println!("All elements:");
    for row_iter in array.rows_iter() {
        for element in row_iter {
            print!("{} ", element);
        }
        println!();
    }
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let width = input.lines().next().unwrap().chars().count() + 1;
    let height = input.lines().count() + 1;
    let mut grid: Array2D<char> = Array2D::filled_with(' ', width, height);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .for_each(|(x, c)| _ = grid.set(x + 1, y + 1, c));
    });
    let mut total = 0;
    for x in 1..width {
        for y in 1..height {
            let pos = (x, y);
            if *grid.get(pos.0, pos.1).unwrap() == ' ' {
                continue;
            }
            total += process_area(&mut grid, pos);
        }
    }
    total.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let width = input.lines().next().unwrap().chars().count() + 1;
    let height = input.lines().count() + 1;
    let mut grid: Array2D<char> = Array2D::filled_with(' ', width, height);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .for_each(|(x, c)| _ = grid.set(x + 1, y + 1, c));
    });
    let mut total = 0;
    for x in 1..width {
        for y in 1..height {
            let pos = (x, y);
            if *grid.get(pos.0, pos.1).unwrap() == ' ' {
                continue;
            }
            total += process_area_part2(&mut grid, pos);
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
        assert_eq!(result, "1930"); // Replace with the actual expected result
    }

    #[test]
    fn test_example2() {
        let input = fs::read_to_string("example2.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "772"); // Replace with the actual expected result
    }

    #[test]
    fn test_example3_part2() {
        let input = fs::read_to_string("example3.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "80"); // Replace with the actual expected result
    }

    #[test]
    fn test_example_e_part2() {
        let input = fs::read_to_string("exampleE.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "236"); // Replace with the actual expected result
    }
    #[test]
    fn test_example2_part2() {
        let input = fs::read_to_string("example2.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "436"); // Replace with the actual expected result
    }

    #[test]
    fn test_example4_part2() {
        let input = fs::read_to_string("example4.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "368"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "1206"); // Replace with the actual expected result
    }
}
