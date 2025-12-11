use std::{collections::HashMap, hash::RandomState};

use nom::{IResult, Parser, bytes::complete::tag, character::complete::{alpha1, line_ending, space1}, multi::separated_list1, sequence::separated_pair};
use petgraph::{Graph, adj::NodeIndex, algo, graph::DiGraph};

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
    let paths = algo::all_simple_paths::<Vec<_>, _, RandomState>(&g, start, target, 0, None).count();
    paths.to_string()
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
    let start = *map.get(&"svr".to_string()).unwrap();
    let target = *map.get(&"out".to_string()).unwrap();
    let paths = algo::all_simple_paths::<Vec<_>, _, RandomState>(&g, start, target, 0, None);
    let mut count = 0;
    for (i,path) in paths.enumerate() {
        println!("{}",i);
        if let Some(_contains_dac) = path.iter().find(|&s|  {g.node_weight(*s).unwrap()=="dac"} ) {
            if let Some(_contains_dac) = path.iter().find(|&s|  {g.node_weight(*s).unwrap()=="fft"} ){
                count+=1;
            }
        }
    }
    count.to_string()
}

fn num_paths(src:NodeIndex, dst:NodeIndex, graph: &Graph<String,i32>) -> usize {
    0
}


pub fn part2_smart(input: &str) -> String {
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
    let start = *map.get(&"svr".to_string()).unwrap();
    let target = *map.get(&"out".to_string()).unwrap();
    num_paths(start, target, &g);
    todo!()
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
