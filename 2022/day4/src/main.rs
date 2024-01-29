struct Range {
    start: usize,
    end: usize, // Inclusive
}

impl Range {
    fn contains(&self, n: usize) -> bool {
        n >= self.start && n <= self.end
    }

    fn from_str(input: &str) -> Range {
        let (start, end) = input.split_once('-').unwrap();
        let start = start.parse::<usize>().unwrap();
        let end = end.parse::<usize>().unwrap();
        Range { start, end }
    }
}

fn solve1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (first_range, second_range) = line.split_once(',').expect("split");
            let first_range = Range::from_str(first_range);
            let second_range = Range::from_str(second_range);
            if (first_range.contains(second_range.start) && first_range.contains(second_range.end))
                || (second_range.contains(first_range.start)
                    && second_range.contains(first_range.end))
            {
                return 1;
            }

            0
        })
        .sum()
}
fn solve2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (first_range, second_range) = line.split_once(',').expect("split");
            let first_range = Range::from_str(first_range);
            let second_range = Range::from_str(second_range);
            if (first_range.contains(second_range.start) || first_range.contains(second_range.end))
                || (second_range.contains(first_range.start)
                    || second_range.contains(first_range.end))
            {
                return 1;
            }

            0
        })
        .sum()
}
fn main() {
    let input = include_str!("../input.txt");
    let result = solve1(input);
    println!("Result 1 : {}", result);

    let result = solve2(input);

    println!("Result 2 : {}", result);
}
