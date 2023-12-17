/*
Example Input :
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533

Each city block is marked by a single digit that represents the amount of heat loss if the crucible enters that block. The starting point, the lava pool, is the top-left city block; the destination, the machine parts factory, is the bottom-right city block. (Because you already start in the top-left block, you don't incur that block's heat loss unless you leave that block and then return to it.)

Because it is difficult to keep the top-heavy crucible going in a straight line for very long, it can move at most three blocks in a single direction before it must turn 90 degrees left or right. The crucible also can't reverse direction; after entering each city block, it may only turn left, continue straight, or turn right.

Directing the crucible from the lava pool to the machine parts factory, but not moving more than three consecutive blocks in the same direction, what is the least heat loss it can incur?
*/

use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::{stable_graph::NodeIndex, Graph};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn parse(input: &str) -> (Graph<i32, Direction>, Vec<Vec<NodeIndex>>) {
    let mut graph: Graph<i32, Direction> = Graph::default();
    let mut nodes = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut row_nodes = vec![];
        for (x, c) in line.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                let node = graph.add_node(digit as i32);
                row_nodes.push(node);
                print!("{}", digit);
            } else {
                panic!("Unexpected character {}", c);
            }
        }
        nodes.push(row_nodes);
        println!();
    }
    let height = nodes.len();
    let width = nodes[0].len();
    println!("W : {} H : {}", width, height);

    for (y, row_nodes) in nodes.iter().enumerate() {
        for (x, node_index) in row_nodes.iter().enumerate() {
            if y > 0 {
                graph.add_edge(*node_index, nodes[y - 1][x], Direction::North);
            }
            if x > 0 {
                graph.add_edge(*node_index, nodes[y][x - 1], Direction::West);
            }
            if y < height - 1 {
                graph.add_edge(*node_index, nodes[y + 1][x], Direction::South);
            }
            if x < width - 1 {
                graph.add_edge(*node_index, nodes[y][x + 1], Direction::East);
            }
        }
    }

    (graph, nodes)
}

fn solve(input: &str) -> i32 {
    let (graph, nodes) = parse(input);

    let width = nodes[0].len();
    let height = nodes.len();
    explore(
        &graph,
        nodes[0][0],
        nodes[height - 1][width - 1],
        0,
        &VecDeque::new(),
        &mut HashSet::new(),
    )
}

fn explore(
    graph: &Graph<i32, Direction>,
    start: NodeIndex,
    target: NodeIndex,
    dist: i32,
    last_directions: &VecDeque<Direction>,
    visited: &mut HashSet<(NodeIndex, Direction)>,
) -> i32 {
    if start == target {
        return dist;
    }
    let mut invalid_directions = vec![];
    if let Some(last_direction) = last_directions.back() {
        if visited.contains(&(start, *last_direction)) {
            return i32::MAX;
        }
        visited.insert((start, *last_directions.back().unwrap()));
        invalid_directions.push(*last_direction);
    }
    if last_directions.len() == 3
        && last_directions[0] == last_directions[1]
        && last_directions[1] == last_directions[2]
    {
        invalid_directions.push(last_directions[0]);
    }

    let mut smallest_distance: i32 = i32::MAX;
    for neighbor in graph.neighbors(start) {
        let edge = graph.find_edge(start, neighbor).unwrap();
        let direction = graph.edge_weight(edge).unwrap();
        if invalid_directions.contains(direction) {
            continue;
        }
        let current_distance = dist + graph.node_weight(neighbor).expect("Neighbor ndoe weight");
        let mut last_directions = last_directions.clone();
        last_directions.pop_front();
        last_directions.push_back(*direction);
        let d = explore(
            graph,
            neighbor,
            target,
            current_distance,
            &last_directions,
            visited,
        );
        if d < smallest_distance {
            smallest_distance = d;
        }
    }
    smallest_distance
}

fn main() {
    println!("Hello, world!");
    let input = include_str!("../../example.txt");
    let result = solve(input);
    println!("Result : {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let example = include_str!("../../example.txt");
        assert_eq!(solve(example), 102);
    }
}
