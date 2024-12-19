use std::collections::HashMap;

// Flag to early stop in part 1
#[derive(Eq, PartialEq)]
enum Part {
    Part1,
    Part2,
}

// Struct to hold cache
struct PatternMatcher<'a> {
    avail_towels: Vec<&'a str>,
    cache: HashMap<&'a str, usize>,
    part: Part,
}

impl<'a> PatternMatcher<'a> {
    fn new(towels: Vec<&'a str>, part: Part) -> Self {
        PatternMatcher {
            avail_towels: towels,
            cache: HashMap::new(),
            part,
        }
    }

    fn is_possible(&mut self, pattern: &'a str) -> usize {
        if pattern.is_empty() {
            return 1;
        }

        if let Some(cached) = self.cache.get(pattern) {
            return *cached;
        }
        let mut ways_to_arange = 0;
        for i in 0..self.avail_towels.len() {
            let towel = self.avail_towels[i];
            if pattern.starts_with(towel) {
                let possible_ways = self.is_possible(&pattern[towel.len()..]);

                // Early stopping for part 1
                if self.part == Part::Part1 && possible_ways > 0 {
                    self.cache.insert(pattern, 1);
                    return 1;
                }
                ways_to_arange += possible_ways;
            }
        }

        self.cache.insert(pattern, ways_to_arange);
        ways_to_arange
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
    let mut pattern_matcher = PatternMatcher::new(avail_towels, Part::Part1);
    let total = wanted_patterns
        .iter()
        .filter(|&pat| pattern_matcher.is_possible(pat) > 0)
        .count();
    total.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let (avail_towels, wanted_patterns) = parse(input);
    let mut pattern_matcher = PatternMatcher::new(avail_towels, Part::Part2);
    let total: usize = wanted_patterns
        .iter()
        .map(|&pat| pattern_matcher.is_possible(pat))
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
        assert_eq!(result, "6"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "16"); // Replace with the actual expected result
    }
}
