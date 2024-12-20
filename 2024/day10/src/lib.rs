use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

fn get_reachable_hilltops(pos: IVec2, map: &HashMap<IVec2, u32>) -> HashSet<IVec2> {
    if let Some(current_tile) = map.get(&pos) {
        if *current_tile == 9 {
            let mut set = HashSet::new();
            set.insert(pos);
            return set;
        }

        let directions = vec![IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y];
        let mut set: HashSet<IVec2> = HashSet::new();
        directions.iter().for_each(|dir| {
            let new_pos = pos + dir;
            if let Some(next_tile) = map.get(&new_pos) {
                if *next_tile == current_tile + 1 {
                    set.extend(get_reachable_hilltops(new_pos, map));
                }
            }
        });
        return set;
    } else {
        return HashSet::new();
    }
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let mut map: HashMap<IVec2, u32> = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            _ = map.insert(
                IVec2 {
                    x: x as i32,
                    y: y as i32,
                },
                c.to_digit(10).unwrap(),
            )
        })
    });

    let total: usize = map
        .iter()
        .map(|(pos, height)| {
            if *height == 0 {
                get_reachable_hilltops(*pos, &map).len()
            } else {
                0
            }
        })
        .sum();
    total.to_string()
}

#[derive(Debug)]
struct Tile {
    height: usize,
    paths: usize,
}

pub fn part2(input: &str) -> String {
    let input = input.trim();
    let mut map: HashMap<IVec2, Tile> = HashMap::new();
    let mut queue: VecDeque<IVec2> = VecDeque::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let height = c.to_digit(10).unwrap() as usize;
            let tile = Tile { height, paths: 0 };
            _ = map.insert(
                IVec2 {
                    x: x as i32,
                    y: y as i32,
                },
                tile,
            );
            if height == 0 {
                queue.push_back(IVec2 {
                    x: x as i32,
                    y: y as i32,
                });
            }
        })
    });

    while let Some(position) = queue.pop_front() {
        let tile = map.get_mut(&position).unwrap();
        let height = tile.height;
        tile.paths += 1;
        let directions = vec![IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y];
        directions.iter().for_each(|dir| {
            let new_pos = position + dir;
            if let Some(next_tile) = map.get(&new_pos) {
                if next_tile.height == height + 1 {
                    queue.push_back(new_pos);
                }
            }
        });
    }

    let total: usize = map
        .iter()
        .map(|(_pos, tile)| if tile.height == 9 { tile.paths } else { 0 })
        .sum();
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
        assert_eq!(result, "36"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "81"); // Replace with the actual expected result
    }
}
