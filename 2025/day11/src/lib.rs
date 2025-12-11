use std::collections::HashMap;

use nom::{IResult, Parser, bytes::complete::tag, character::complete::{alpha1, line_ending, space1}, multi::separated_list1, sequence::separated_pair};
use petgraph::{Graph, algo::toposort, graph::DiGraph};

#[derive(Debug)]
struct Line {
    source : String,
    destination : Vec<String>
}

fn parse_line(input: &str)-> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(alpha1, tag(": "), 
    separated_list1( space1, alpha1)).parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines)=separated_list1(line_ending, parse_line).parse(input)?;
    let lines = lines.iter().map(|(s,d)| {
        let source = s.to_string();
        let d = d.iter().map(|d| d.to_string()).collect::<Vec<String>>();
        Line {source : source, destination :d}
    }).collect::<Vec<Line>>();
    Ok((input,lines))
}


pub fn part1(input: &str) -> String {
    let input = input.trim();
    let mut g = DiGraph::<String,i32>::new();
    let (_input, lines) = parse(input).unwrap();
    let mut map  = HashMap::new();

    for line in lines.iter()  {
        map.entry(line.source.clone()).or_insert(g.add_node(line.source.clone()));
    }
    map.entry("out".to_string()).or_insert(g.add_node("out".to_string()));
    for line in lines.iter()  {
        let src = map.get(&line.source).unwrap();
        for d in line.destination.iter() {
            let dst = map.get(d).unwrap();

            g.add_edge(*src, *dst, 1);
        }
    }
    
    let start = *map.get(&"you".to_string()).unwrap();
    let target = *map.get(&"out".to_string()).unwrap();
    let paths = count_paths_dag(&g, start,target);
    paths.to_string()
}


fn num_paths(src:&str, dst:&str, graph: &Graph<String,i32>, node_map :&HashMap<String,petgraph::graph::NodeIndex>) -> u128 {
    
    let start = *node_map.get(&src.to_string()).unwrap();
    let target = *node_map.get(&dst.to_string()).unwrap();

    let paths = count_paths_dag(graph,  start,target);
    //println!("{} -> {} : {}",src,dst,paths);
    paths
}

fn count_paths_dag(
    graph: &DiGraph<String, i32>,
    src: petgraph::graph::NodeIndex,
    dst: petgraph::graph::NodeIndex,
) -> u128 {
    // Topological sort; will return Err if there is a cycle.
    let topo = toposort(graph, None)
        .expect("Graph is not a DAG (cycle detected)");

    // dp[v] = number of paths from src to v
    let mut dp = vec![0u128; graph.node_count()];
    dp[src.index()] = 1;

    for node in topo {
        let ways = dp[node.index()];
        if ways == 0 {
            // No paths reach this node from src, skip
            continue;
        }

        for succ in graph.neighbors_directed(node, petgraph::Direction::Outgoing) {
            dp[succ.index()] += ways;
        }
    }

    dp[dst.index()]
}


pub fn part2(input: &str) -> String {
    let input = input.trim();
    let mut g = DiGraph::<String,i32>::new();
    let (_input, lines) = parse(input).unwrap();
    let mut map  = HashMap::new();

    for line in lines.iter()  {
        map.entry(line.source.clone()).or_insert(g.add_node(line.source.clone()));
    }
    map.entry("out".to_string()).or_insert(g.add_node("out".to_string()));
    for line in lines.iter()  {
        let src = map.get(&line.source).unwrap();
        for d in line.destination.iter() {
            let dst = map.get(d).unwrap();

            g.add_edge(*src, *dst, 1);
        }
    }
    let svr_to_fft = num_paths("svr", "fft", &g, &map);
    let fft_to_dac = num_paths("fft", "dac", &g, &map);
    let dac_to_out = num_paths("dac", "out", &g, &map);


    // SVR -> FFT -> DAC -> OUT
    let r1 = svr_to_fft * fft_to_dac * dac_to_out;

    let svr_to_dac = num_paths("svr", "dac", &g, &map);
    let dac_to_fft = num_paths("dac", "fft", &g, &map);
    let fft_to_out = num_paths("fft", "out", &g, &map);
    // SVR -> DAC -> FFT -> OUT
    let r2 = svr_to_dac * dac_to_fft * fft_to_out;

    (r1+r2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "5"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example2.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "2"); // Replace with the actual expected result
    }
}
