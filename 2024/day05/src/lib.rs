use std::collections::{HashMap, HashSet};

type Page = usize;

// Verifies using the rules and returns the middle page
fn verify_update(line: &str, update_rules: &HashMap<Page, Vec<Page>>) -> Page {
    let mut disallowed_updates: HashSet<Page> = HashSet::new();
    let pages: Vec<Page> = line
        .split(',')
        .map(|page| page.parse::<Page>().unwrap())
        .collect();
    for page in pages.iter() {
        if disallowed_updates.contains(&page) {
            return 0;
        }
        if let Some(pages_previous) = update_rules.get(&page) {
            for prev in pages_previous.iter() {
                disallowed_updates.insert(*prev);
            }
        }
    }

    pages[pages.len() / 2]
}

#[derive(Debug)]
struct Graph {
    nodes: HashSet<Page>,
    edges: HashMap<Page, HashSet<Page>>,
    reverse_edges: HashMap<Page, HashSet<Page>>,
}
impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
        }
    }
    fn add_node(&mut self, value: Page) {
        self.nodes.insert(value);
        self.edges.insert(value, HashSet::new());
        self.reverse_edges.insert(value, HashSet::new());
    }
    fn add_edge(&mut self, from: Page, to: Page) {
        assert!(self.nodes.contains(&from) && self.nodes.contains(&to));
        self.edges.entry(from).and_modify(|v| _ = v.insert(to));
        self.reverse_edges
            .entry(to)
            .and_modify(|v| _ = v.insert(from));
    }

    fn topologic_sort(&mut self) -> Vec<Page> {
        // We just need to sort the first half
        let len = self.nodes.len() / 2 + 1;
        let mut result: Vec<Page> = Vec::new();
        while result.len() != len {
            let (&starting_point, _) = self
                // Find node with no outgoing edges (no pages need to come before it)
                .edges
                .iter()
                .find(|(_, v)| v.is_empty())
                .expect("Should find starting point.");

            result.push(starting_point);
            // Iterate over all incoming edges (pages that depend on this page) and delete them
            for edge in self.reverse_edges.get(&starting_point).unwrap() {
                self.edges
                    .entry(*edge)
                    .and_modify(|edges| _ = edges.remove(&starting_point));
            }
            // Delete this node from the graph
            self.nodes.remove(&starting_point);
            self.edges.remove(&starting_point);
            self.reverse_edges.remove(&starting_point);
        }
        result
    }
}

fn fix_update(line: &str, update_rules: &HashMap<Page, Vec<Page>>) -> Page {
    let pages: HashSet<Page> = line
        .split(',')
        .map(|page| page.parse::<Page>().unwrap())
        .collect();
    let mut graph: Graph = Graph::new();
    for page in pages.iter() {
        graph.add_node(*page);
    }
    for (page, previous) in update_rules.iter() {
        if !pages.contains(page) {
            continue;
        }
        for prev in previous.iter() {
            if pages.contains(prev) {
                graph.add_edge(*page, *prev);
            }
        }
    }
    let sort = graph.topologic_sort();
    sort[sort.len() - 1]
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let (rules, updates) = input
        .split_once("\n\n")
        .expect("Expect input to have 2 parts.");

    let mut update_rules: HashMap<Page, Vec<Page>> = HashMap::new();
    rules.lines().for_each(|line| {
        let (prev, page) = line.split_once('|').unwrap();
        let prev = prev.parse::<Page>().unwrap();
        let page = page.parse::<Page>().unwrap();
        let v = update_rules.entry(page).or_insert(Vec::new());
        v.push(prev);
    });
    let total: Page = updates
        .lines()
        .map(|line| verify_update(line, &update_rules))
        .sum();

    total.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let (rules, updates) = input
        .split_once("\n\n")
        .expect("Expect input to have 2 parts.");

    let mut update_rules: HashMap<Page, Vec<Page>> = HashMap::new();
    rules.lines().for_each(|line| {
        let (prev, page) = line.split_once('|').unwrap();
        let prev = prev.parse::<Page>().unwrap();
        let page = page.parse::<Page>().unwrap();
        let v = update_rules.entry(page).or_insert(Vec::new());
        v.push(prev);
    });

    let wrong_updates = updates
        .lines()
        .filter(|line| verify_update(line, &update_rules) == 0)
        .collect::<Vec<&str>>();
    let total: Page = wrong_updates
        .iter()
        .map(|line| fix_update(line, &update_rules))
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
        assert_eq!(result, "143"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "123"); // Replace with the actual expected result
    }
}
