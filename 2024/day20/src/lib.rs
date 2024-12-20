use std::collections::{HashMap, VecDeque};

use glam::IVec2;

#[derive(Debug, PartialEq, Eq)]
enum TileType {
    Wall,
    Empty,
}

#[derive(Debug)]
struct Tile {
    cost: i64,
    kind: TileType,
}
// Start, end, map
fn parse(input: &str) -> (IVec2, IVec2, HashMap<IVec2, Tile>) {
    let mut map = HashMap::new();
    let mut start = IVec2 { x: 0, y: 0 };
    let mut end = IVec2 { x: 0, y: 0 };
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let tile = Tile {
                cost: i64::MAX,
                kind: if c == '#' {
                    TileType::Wall
                } else {
                    TileType::Empty
                },
            };
            let pos = IVec2 {
                x: x as i32,
                y: y as i32,
            };
            if c == 'S' {
                start = pos;
            } else if c == 'E' {
                end = pos;
            }
            map.insert(pos, tile);
        });
    });
    (start, end, map)
}

fn bfs(start: IVec2, map: &mut HashMap<IVec2, Tile>) {
    let mut queue: VecDeque<(IVec2, i64)> = VecDeque::new();
    let directions = vec![IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];
    queue.push_back((start, 0));
    while let Some((pos, cost)) = queue.pop_front() {
        let current_tile = map.get_mut(&pos).unwrap();
        if current_tile.cost <= cost {
            continue;
        } else {
            current_tile.cost = cost;
        }

        for dir in directions.iter() {
            let new_pos = pos + dir;
            if let Some(tile) = map.get(&new_pos) {
                match tile.kind {
                    TileType::Wall => continue,
                    TileType::Empty => {
                        let cost = cost + 1;
                        if tile.cost <= cost {
                            continue;
                        }
                        queue.push_back((new_pos, cost));
                    }
                };
            }
        }
    }
}

fn find_cheats(map: &HashMap<IVec2, Tile>) -> HashMap<i64, usize> {
    let max_x = map.iter().map(|(k, _)| k.x).max().unwrap();
    let max_y = map.iter().map(|(k, _)| k.y).max().unwrap();
    let mut cheats: HashMap<i64, usize> = HashMap::new();
    let dirs = vec![
        IVec2 { x: 1, y: 1 },
        IVec2 { x: -1, y: 1 },
        IVec2 { x: 1, y: -1 },
        IVec2 { x: -1, y: -1 },
        IVec2 { x: 2, y: 0 },
        IVec2 { x: -2, y: 0 },
        IVec2 { x: 0, y: 2 },
        IVec2 { x: 0, y: -2 },
    ];
    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = IVec2 { x, y };
            let tile = map.get(&pos).unwrap();
            match tile.kind {
                TileType::Wall => {}
                TileType::Empty => {
                    let startcost = tile.cost;
                    for dir in dirs.iter() {
                        let new_pos = pos + dir;
                        if let Some(end_tile) = map.get(&new_pos) {
                            if end_tile.kind == TileType::Wall {
                                continue;
                            }
                            let end_cost = end_tile.cost;
                            let savings = startcost - end_cost - 2;
                            if savings > 0 {
                                cheats.entry(savings).and_modify(|x| *x += 1).or_insert(1);
                            }
                        }
                    }
                }
            };
        }
    }
    cheats
}

fn find_cheats_part_2(map: &HashMap<IVec2, Tile>) -> HashMap<i64, usize> {
    let max_x = map.iter().map(|(k, _)| k.x).max().unwrap();
    let max_y = map.iter().map(|(k, _)| k.y).max().unwrap();
    let mut cheats: HashMap<i64, usize> = HashMap::new();
    let mut dirs = Vec::new();
    for x in -20..=20 {
        for y in -20..=20 {
            let dir = IVec2 { x, y };
            if dir.x.abs() + dir.y.abs() <= 20 {
                dirs.push(dir);
            }
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = IVec2 { x, y };
            let tile = map.get(&pos).unwrap();
            match tile.kind {
                TileType::Wall => {}
                TileType::Empty => {
                    let startcost = tile.cost;
                    for dir in dirs.iter() {
                        let new_pos = pos + dir;
                        if let Some(end_tile) = map.get(&new_pos) {
                            if end_tile.kind == TileType::Wall {
                                continue;
                            }
                            let end_cost = end_tile.cost;
                            let savings =
                                startcost - end_cost - dir.x.abs() as i64 - dir.y.abs() as i64;
                            if savings > 0 {
                                cheats.entry(savings).and_modify(|x| *x += 1).or_insert(1);
                            }
                        }
                    }
                }
            };
        }
    }
    cheats
}

#[allow(dead_code)]
fn print_map(map: &HashMap<IVec2, Tile>) {
    let max_x = map.iter().map(|(k, _)| k.x).max().unwrap();
    let max_y = map.iter().map(|(k, _)| k.y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let tile = map.get(&IVec2 { x, y }).unwrap();
            match tile.kind {
                TileType::Wall => print!("#"),
                TileType::Empty => print!("{}", tile.cost % 10),
            };
        }
        println!();
    }
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let (_, end, mut map) = parse(input);

    bfs(end, &mut map);
    let cheats = find_cheats(&map);
    let total: usize = cheats
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, v)| v)
        .sum();
    total.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let (_, end, mut map) = parse(input);

    bfs(end, &mut map);
    let cheats = find_cheats_part_2(&map);
    let total: usize = cheats
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, v)| v)
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
        assert_eq!(result, "0"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "0"); // Replace with the actual expected result
    }
}
