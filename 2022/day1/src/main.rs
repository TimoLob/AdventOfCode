fn solve1(input: &str) -> i32 {
    let elves = input.split("\n\n");
    let calories = elves.map(|e| {
        e.lines()
            .fold(0, |acc, c| acc + c.parse::<i32>().expect(""))
    });
    calories.max().expect("")
}

fn solve2(input: &str) -> i32 {
    let elves = input.split("\n\n");
    let mut calories = elves
        .map(|e| {
            e.lines()
                .fold(0, |acc, c| acc + c.parse::<i32>().expect(""))
        })
        .collect::<Vec<i32>>();
    calories.sort();
    let (_, top3) = calories.split_at(calories.len() - 3);
    top3.iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let task1 = solve1(input);
    println!("Solution 1 : {}", task1);
    let task2 = solve2(input);
    println!("Solution 2 : {}", task2);
}
