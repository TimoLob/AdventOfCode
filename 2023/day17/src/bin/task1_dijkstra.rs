use core::fmt;
use petgraph::{stable_graph::NodeIndex, Graph};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct State {
    dist: i32,
    position: NodeIndex,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "N",
                Direction::East => "E",
                Direction::West => "W",
                Direction::South => "S",
            }
        )
    }
}

fn solve(input: &str) -> i32 {
    let (graph, nodes) = parse(input);

    return dijkstra(graph, nodes);
}

fn dijkstra(graph: Graph<i32, Direction>, nodes: Vec<Vec<NodeIndex>>) -> i32 {
    let mut dist: HashMap<NodeIndex, i32> = HashMap::new();
    let mut prev: HashMap<NodeIndex, Option<NodeIndex>> = HashMap::new();
    let width = nodes[0].len();
    let height = nodes.len();
    let start = nodes[0][0];
    let target = nodes[height - 1][width - 1];
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    for row in nodes.iter() {
        for &index in row {
            dist.insert(index, i32::MAX);
            prev.insert(index, None);
            if index == start {
                dist.insert(index, 0);
            }
            pq.push(State {
                dist: dist[&index],
                position: index,
            });
        }
    }

    while !pq.is_empty() {
        let u = pq.pop().expect("PQ should always have one element here");
        let u_index = u.position;
        // Get Last 3 directions
        let mut last_directions = Vec::with_capacity(3);
        let mut current = u_index;
        let mut current_prev: NodeIndex;
        for _ in 0..3 {
            let opt = prev[&current];
            if opt.is_none() {
                break;
            }
            current_prev = opt.unwrap();
            let edge = graph
                .find_edge(current_prev, current)
                .expect("Should be valid edge");
            let dir = graph.edge_weight(edge).expect("Edge should have weight");
            last_directions.push(*dir);
            current = current_prev;
        }
        let mut invalid_direction: Option<Direction> = None;
        if last_directions.len() == 3
            && last_directions[0] == last_directions[1]
            && last_directions[1] == last_directions[2]
        {
            invalid_direction = Some(last_directions[0]);
        }

        for v in graph.neighbors(u_index) {
            let edge = graph
                .find_edge(u_index, v)
                .expect("Should be an edge since neightbors");
            let edge_dir = graph.edge_weight(edge).unwrap();
            if invalid_direction.is_some_and(|x| x == *edge_dir) {
                continue;
            }
            let alt = dist[&u_index] + graph.node_weight(v).expect("Node should have weight");
            if alt < dist[&v] {
                dist.insert(v, alt);
                prev.insert(v, Some(u_index));
                pq.retain(|x| x.position != v);
                pq.push(State {
                    dist: alt,
                    position: v,
                });
                //println!("{:?}", pq);
            }
        }
    }
    println!("{:?}", dist);
    println!("{:?}", prev);
    let mut current = target;
    print!("{:?}->", current);
    while let Some(p) = prev[&current] {
        let cost = graph.node_weight(p).expect("msg");
        print!("{:?}[{}]->", p, cost);
        current = p;
    }
    println!();
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
    println!("Hello, world!");
    let input = include_str!("../../input.txt");
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
