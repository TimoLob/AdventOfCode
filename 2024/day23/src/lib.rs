use petgraph::graph::UnGraph;
use std::collections::{HashMap, HashSet}; // For combinations // For combinations

fn parse(input: &str) -> (Vec<&str>, Vec<(u32, u32)>) {
    let edges_in = input.lines().map(|line| line.split_once('-').unwrap());
    let mut edges: Vec<(u32, u32)> = Vec::new();
    let mut map: HashMap<&str, u32> = HashMap::new();
    let mut index = 0;
    edges_in.for_each(|(a, b)| {
        if !map.contains_key(a) {
            map.insert(a, index);
            index += 1;
        }
        if !map.contains_key(b) {
            map.insert(b, index);
            index += 1;
        }
        let a_index = *map.get(a).unwrap();
        let b_index = *map.get(b).unwrap();
        edges.push((a_index, b_index));
    });
    let mut nodes: Vec<&str> = Vec::new();
    nodes.resize_with(map.len(), || "");
    map.iter().for_each(|(&k, &v)| nodes[v as usize] = k);
    (nodes, edges)
}

pub fn part1(input: &str) -> String {
    let input = input.trim();

    let (nodes, edges) = parse(input);
    let g = UnGraph::<(), ()>::from_edges(edges);

    let nodes_starting_with_t: HashSet<usize> = nodes
        .iter()
        .enumerate()
        .filter(|(_idx, x)| x.starts_with('t'))
        .map(|(idx, _)| idx)
        .collect();

    let mut subgraphs: HashSet<(usize, usize, usize)> = HashSet::new();
    for a in g.node_indices() {
        for b in g.neighbors(a) {
            for c in g.neighbors(a) {
                if g.contains_edge(b, c) {
                    let mut nodes = vec![a.index(), b.index(), c.index()];
                    nodes.sort();
                    subgraphs.insert((nodes[0], nodes[1], nodes[2]));
                }
            }
        }
    }

    let total = subgraphs
        .iter()
        .filter(|(a, b, c)| {
            nodes_starting_with_t.contains(a)
                || nodes_starting_with_t.contains(b)
                || nodes_starting_with_t.contains(c)
        })
        .count();
    total.to_string()
}

/// Recursive function implementing the Bron–Kerbosch algorithm
fn bron_kerbosch(
    graph: &UnGraph<(), ()>,
    mut r: Vec<u32>,
    mut p: HashSet<u32>,
    mut x: HashSet<u32>,
    cliques: &mut Vec<Vec<u32>>,
) {
    if p.is_empty() && x.is_empty() {
        // Found a maximal clique
        cliques.push(r.clone());
        return;
    }

    // Iterate over a copy of p since we'll be modifying it
    for &node in p.clone().iter() {
        // Add node to the current clique (R)
        r.push(node);

        // Neighbors of the current node
        let neighbors: HashSet<u32> = graph
            .neighbors(node.into())
            .map(|x| x.index() as u32)
            .collect();

        // Recursively call Bron–Kerbosch with updated sets
        bron_kerbosch(
            graph,
            r.clone(),
            &p & &neighbors, // P ∩ N(v)
            &x & &neighbors, // X ∩ N(v)
            cliques,
        );

        // Backtrack
        r.pop();
        p.remove(&node);
        x.insert(node);
    }
}

/// Find the maximum clique using the Bron–Kerbosch algorithm
fn find_maximum_clique(graph: &UnGraph<(), ()>) -> Vec<u32> {
    let mut cliques = Vec::new();
    let nodes: HashSet<u32> = graph.node_indices().map(|x| x.index() as u32).collect();

    bron_kerbosch(
        graph,
        Vec::new(),
        nodes.clone(),
        HashSet::new(),
        &mut cliques,
    );

    // Find the largest clique
    cliques
        .into_iter()
        .max_by_key(|c| c.len())
        .unwrap_or_default()
}

pub fn part2(input: &str) -> String {
    let input = input.trim();
    let (nodes, edges) = parse(input);
    let g = UnGraph::<(), ()>::from_edges(edges);

    let largest_clique = find_maximum_clique(&g);
    let mut names: Vec<&str> = largest_clique
        .iter()
        .map(|idx| nodes[*idx as usize])
        .collect();
    names.sort();

    names.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "7"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "co,de,ka,ta"); // Replace with the actual expected result
    }
}
