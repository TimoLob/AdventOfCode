use std::{collections::HashMap, fs, hash::Hash};
pub fn part1(input: &str) -> String {
    let lines = input.split('\n');
    let mut left_list = lines
        .clone()
        .map(|x| x.split("   ").next().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut right_list = lines
        .map(|x| x.split("   ").nth(1).unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    left_list.sort();
    right_list.sort();

    let difference = left_list
        .iter()
        .zip(right_list.iter())
        .map(|tuple| (tuple.0 - tuple.1).abs())
        .sum::<i32>();

    difference.to_string()
}

pub fn part2(input: &str) -> String {
    let lines = input.split('\n');
    let left_list = lines
        .clone()
        .map(|x| x.split("   ").next().unwrap().parse::<i32>().unwrap());
    let mut map: HashMap<i32, i32> = HashMap::new();
    let right_list = lines.map(|x| x.split("   ").nth(1).unwrap().parse::<i32>().unwrap());
    right_list.for_each(|x| *map.entry(x).or_insert_with(|| 0) += 1);
    // println!("{:?}", map);
    let similarity_score = left_list
        .map(|x| x * map.get(&x).or(Some(&0)).expect(""))
        .sum::<i32>();
    similarity_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "11"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "31"); // Replace with the actual expected result
    }
}
