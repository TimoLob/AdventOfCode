use std::collections::HashMap;

use rayon::prelude::*;

struct PatternMatcher<'a> {
    avail_towels: Vec<&'a str>,
    cache: HashMap<&'a str, bool>,
}

impl<'a> PatternMatcher<'a> {
    fn new(towels: Vec<&'a str>) -> Self {
        PatternMatcher {
            avail_towels: towels,
            cache: HashMap::new(),
        }
    }

    fn is_possible(&mut self, pattern: &'a str) -> bool {
        if pattern.is_empty() {
            return true;
        }

        if let Some(cached) = self.cache.get(pattern) {
            return *cached;
        }
        let towels = self.avail_towels.clone();
        for &towel in towels.iter() {
            if pattern.starts_with(towel) && self.is_possible(&pattern[towel.len()..pattern.len()])
            {
                self.cache.insert(pattern, true);
                return true;
            }
        }

        self.cache.insert(pattern, false);
        false
    }
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (avail_towels, wanted_patterns) = input.split_once("\n\n").unwrap();
    let avail_towels = avail_towels.split(", ").collect();
    let wanted_patterns = wanted_patterns.lines().collect();
    //dbg!(&avail_towels, &wanted_patterns);
    (avail_towels, wanted_patterns)
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let (avail_towels, wanted_patterns) = parse(input);
    let mut pattern_matcher = PatternMatcher::new(avail_towels);
    let total = wanted_patterns
        .iter()
        .filter(|&pat| pattern_matcher.is_possible(pat))
        .count();
    total.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
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
        assert_eq!(result, "6"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, ""); // Replace with the actual expected result
    }
}
