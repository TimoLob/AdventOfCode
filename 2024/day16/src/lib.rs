use std::collections::HashSet;

use glam::IVec2;
use pathfinding::prelude::{astar, yen};

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Node {
    pos: IVec2,
    dir: IVec2,
}

// Start, End, Walls
fn parse(input: &str) -> (IVec2, IVec2, HashSet<IVec2>) {
    let mut walls = HashSet::new();
    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '#' => {
                _ = walls.insert(IVec2 {
                    x: x as i32,
                    y: y as i32,
                })
            }
            'S' => {
                start = IVec2 {
                    x: x as i32,
                    y: y as i32,
                }
            }
            'E' => {
                end = IVec2 {
                    x: x as i32,
                    y: y as i32,
                }
            }
            '.' => {}
            _ => unreachable!("Unexpected char"),
        });
    });
    (start, end, walls)
}

fn successors(node: &Node, walls: &HashSet<IVec2>) -> Vec<(Node, u32)> {
    let directions = vec![IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];
    let mut succ = Vec::new();
    for &dir in directions.iter() {
        let new_pos = node.pos + dir;

        if walls.contains(&new_pos) {
            // Can't walk into walls
            continue;
        }
        if dir == (node.dir * -1) {
            // Can't turn around
            succ.push((Node { pos: new_pos, dir }, 2001));
            continue;
        }
        // Cost = 1 if moving in the same direction, otherwise 1000+1 for turning and moving
        let cost = if dir == node.dir { 1u32 } else { 1001u32 };
        succ.push((Node { pos: new_pos, dir }, cost));
    }

    succ
}

// ======================== Part 1 =================================
// Find the shortest path using A*

fn heuristic(node: &Node, goal: IVec2) -> u32 {
    node.pos.distance_squared(goal) as u32
}
pub fn part1(input: &str) -> String {
    let input = input.trim();
    let (start, end, walls) = parse(input);

    let start = Node {
        pos: start,
        dir: IVec2::X,
    };
    let result = astar(
        &start,
        |n| successors(n, &walls),
        |n| heuristic(n, end),
        |node| node.pos == end,
    )
    .expect("No path found");
    dbg!(result.0.len());
    result.1.to_string()
}
// =================== Part 2 ===========================
// Use yen (https://docs.rs/pathfinding/latest/pathfinding/directed/yen/fn.yen.html) to find the
// k-shortest paths with the same cost
// Then put all the visited positions into a HashSet and return the length
// Very slow

// Returns number of shortest paths with same cost
fn num_shortest_paths(start: &Node, walls: &HashSet<IVec2>, end: IVec2) -> Option<usize> {
    for k in 1..100 {
        // Find k shortest paths
        dbg!(k);
        let result = yen(start, |n| successors(n, &walls), |node| node.pos == end, k);
        // If there is one path with different cost, return k-1
        let cost = result[0].1;
        for (_, c) in result.iter() {
            if *c != cost {
                return Some(k - 1);
            }
        }
    }
    None
}

// Calculates the k shortest paths and returns the number of unique positions
fn get_unique_positions(start: &Node, walls: &HashSet<IVec2>, end: IVec2, k: usize) -> usize {
    let result = yen(start, |n| successors(n, &walls), |node| node.pos == end, k);
    let mut positions: HashSet<IVec2> = HashSet::new();
    result.iter().for_each(|(nodes, cost)| {
        dbg!(nodes.len(), cost);
        nodes.iter().for_each(|n| {
            positions.insert(n.pos);
        });
    });
    positions.len()
}

pub fn part2(input: &str) -> String {
    let input = input.trim();

    let (start, end, walls) = parse(input);
    let start = Node {
        pos: start,
        dir: IVec2::X,
    };
    let k = num_shortest_paths(&start, &walls, end).expect("Couldn't find all paths");

    get_unique_positions(&start, &walls, end, k).to_string()
}

// =================== Tests ===================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example1_part1() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "7036"); // Replace with the actual expected result
    }

    #[test]
    fn test_example2_part1() {
        let input = fs::read_to_string("example2.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "11048"); // Replace with the actual expected result
    }

    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "45"); // Replace with the actual expected result
    }

    #[test]
    fn test_example2_part2() {
        let input = fs::read_to_string("example2.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "64"); // Replace with the actual expected result
    }
}
