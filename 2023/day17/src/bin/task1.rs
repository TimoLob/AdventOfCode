use std::{
    cmp,
    collections::{BinaryHeap, HashMap},
};

use petgraph::{stable_graph::NodeIndex, Graph};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn dijkstra(graph: &Graph<i32, Direction>, start: NodeIndex, target: NodeIndex) -> i32 {
    let mut priority_queue = BinaryHeap::new();
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    dist.insert(start, 0);
    prev.insert(start, None);
    // PQ ((negative) distance from start, nodeindex, direction, same_steps)
    priority_queue.push((0, start, Direction::South, 0));
    priority_queue.push((0, start, Direction::East, 0));

    while let Some((d, node, last_dir, same_steps)) = priority_queue.pop() {
        for neighbor in graph.neighbors(node) {
            let edge = graph.find_edge(node, neighbor).unwrap();
            let edge_dir = graph.edge_weight(edge).unwrap();
            // No backtracking
            if match last_dir {
                Direction::North => *edge_dir == Direction::South,
                Direction::South => *edge_dir == Direction::North,
                Direction::East => *edge_dir == Direction::West,
                Direction::West => *edge_dir == Direction::East,
            } {
                continue;
            }

            // No 3 steps in the same direction
            let mut same_steps = same_steps;
            if last_dir == *edge_dir {
                same_steps += 1;
            } else {
                same_steps = 0;
            }
            if same_steps > 3 {
                continue;
            }

            let new_dist = dist[&node] + graph[neighbor];
            if !dist.contains_key(&neighbor) || new_dist < dist[&neighbor] {
                dist.insert(neighbor, new_dist);
                prev.insert(neighbor, Some(node));
                // Update priorty queue entry if already there
                let mut found = false;
                let mut new_priority_queue = BinaryHeap::new();
                while let Some((d, n, dir, steps)) = priority_queue.pop() {
                    if n == neighbor && dir == *edge_dir && new_dist < -d {
                        new_priority_queue.push((-new_dist, neighbor, *edge_dir, same_steps));
                        found = true;
                    } else {
                        new_priority_queue.push((d, n, dir, steps));
                    }
                }
                if !found {
                    new_priority_queue.push((-new_dist, neighbor, *edge_dir, same_steps));
                }
                priority_queue = new_priority_queue;
            }
        }
    }
    // Print path based on prev
    let mut node = target;
    let mut path = vec![target];
    while let Some(prev_node) = prev[&node] {
        path.push(prev_node);
        node = prev_node;
    }
    path.reverse();
    println!("Path : {:?}", path);

    return dist[&target];
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

fn main() {
    let input = include_str!("../../input.txt");
    let (graph, nodes) = parse(input);
    let start = nodes[0][0];
    let width = nodes[0].len();
    let height = nodes.len();
    let target = nodes[height - 1][width - 1];
    println!("Start : {:?} Target : {:?}", start, target);
    let result = dijkstra(&graph, start, target);
    println!("Result : {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = include_str!("../../example.txt");
        let (graph, nodes) = parse(input);
        let start = nodes[0][0];
        let width = nodes[0].len();
        let height = nodes.len();
        let target = nodes[height - 1][width - 1];
        println!("Start : {:?} Target : {:?}", start, target);
        let result = dijkstra(&graph, start, target);
        println!("Result : {}", result);
        assert_eq!(result, 102);
    }
}
