use std::{collections::HashSet, ops::RangeInclusive};

use nom::{
    IResult,
    Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u64 as parse_u64},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

fn is_overlap(a: &RangeInclusive<u64>, b:&RangeInclusive<u64>) -> bool {
    //println!("{:?} {:?}", a, b);
    if a.end() < b.start() { // A strictly to the left of B
        return false;
    }
    if a.start() > b.end() { // A strictly to the right
        return false;
    }
    true
}

fn combine_overlapping_ranges(a: &RangeInclusive<u64>, b:&RangeInclusive<u64>) -> RangeInclusive<u64> {
    debug_assert!(is_overlap(a, b));
    let result = *a.start().min(b.start())..=*a.end().max(b.end());
    //println!("{:?}+{:?} => {:?}", a,b,result);
    result
}


fn parse_ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(
        line_ending,
        map(
            separated_pair(parse_u64, tag("-"), parse_u64),
            |(start, end)| start..=end,
        ),
    ).parse(input)

}

fn parse(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {

    separated_pair(
    parse_ranges,
    tag("\n\n"),
    separated_list1(line_ending, parse_u64)
    ).parse(input)

}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let (_input, (ranges, ingredients)) = parse(input).unwrap();
    
    let total = ingredients.iter().filter(|&ingredient| {
        ranges.iter().any(|range| range.contains(ingredient))
    }).count();
    total.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let (_input, (ranges, _ingredients)) = parse(input).unwrap();

    let mut range_set : HashSet<RangeInclusive<u64>> = HashSet::new();
    for range in ranges {
        let overlapping = range_set.iter().filter_map(|rs| 
            if is_overlap(rs, &range) {
                Some(rs.clone())
            }
            else {
                None
            }
        ).collect::<Vec<RangeInclusive<u64>>>();
        let mut result_range = range.clone();
        for or in overlapping {
            result_range = combine_overlapping_ranges(&or, &result_range);
            range_set.remove(&or);
        }
        range_set.insert(result_range);
    }
    range_set.iter().map(|r| r.end()-r.start()+1 ).sum::<u64>().to_string()    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "3"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "14"); // Replace with the actual expected result
    }

    #[test]
    fn range_overlap() {
        let a = 10..=15u64;
        let b = 2..=5u64; // B to the left
        assert!(!is_overlap(&a, &b));
        let b = 16..=20u64; // B to the right
        assert!(!is_overlap(&a, &b));

        let b = 5..=12u64; // B reaches into A
        assert!(is_overlap(&a, &b));
        let b = 12..=17u64; // B start is in A
        assert!(is_overlap(&a, &b));

        let b = 0..=100u64; // B contains A
        assert!(is_overlap(&a, &b));
        let b = 10..=13u64; // A contains B
        assert!(is_overlap(&a, &b));
    }

    #[test]
    fn combine_ranges() {
        let a = 10..=15u64;

        let b = 5..=12u64; // B reaches into A
        assert_eq!(combine_overlapping_ranges(&a, &b), 5..=15u64);
        let b = 12..=17u64; // B start is in A
        assert_eq!(combine_overlapping_ranges(&a, &b), 10..=17u64);

        let b = 0..=100u64; // B contains A
        assert_eq!(combine_overlapping_ranges(&a, &b), 0..=100u64);
        let b = 10..=13u64; // A contains B
        assert_eq!(combine_overlapping_ranges(&a, &b), 10..=15u64);
    }
}
