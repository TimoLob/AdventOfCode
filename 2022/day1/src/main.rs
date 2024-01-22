fn solve1(input: &str) -> i32 {
    let elves = input.split("\n\n");
    let calories = elves.map(|e| {
        e.lines()
            .fold(0, |acc, c| acc + c.parse::<i32>().expect(""))
    });
    calories.max().expect("")
}

fn main() {
    let input = include_str!("../input.txt");
    let task1 = solve1(input);
    println!("Solution 1 : {}", task1);
}
