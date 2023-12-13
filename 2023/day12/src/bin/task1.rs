fn matches_pattern(line: &Vec<char>, pattern: &Vec<usize>) -> bool {
    let mut current_char = 0usize;
    let mut current_pattern = 0usize;
    let mut counter: usize;
    // println!("{:?}",line);
    // println!("{:?}",pattern);

    while current_char < line.len() && current_pattern < pattern.len() {
        let c = line[current_char];
        // println!("Char : {} Index: {} | pattern : {}",c,current_char,current_pattern);
        if c == '.' {
            current_char += 1;
        }
        if c == '#' {
            counter = 0;
            if current_char + pattern[current_pattern] > line.len() {
                return false;
            }
            for idx in current_char..current_char + pattern[current_pattern] {
                if line[idx] == '#' {
                    counter += 1;
                }
            }
            if counter == pattern[current_pattern] {
                current_char += pattern[current_pattern];
                if current_char < line.len() && line[current_char] == '#' {
                    return false;
                }
                current_pattern += 1;
            } else {
                return false;
            }
        }
    }
    if current_pattern == pattern.len() {
        for idx in current_char..line.len() {
            if line[idx] == '#' {
                return false;
            }
        }
        return true;
    }
    false
}

// ?###???????? 3,2,1
fn solve_line(line: &str) -> usize {
    let split = line.split(" ").collect::<Vec<&str>>();
    let springs = split[0].to_string();
    let pattern_string = split[1];
    let pattern = pattern_string
        .split(",")
        .map(|s| s.parse::<usize>().expect("Should be number"))
        .collect::<Vec<usize>>();
    let mut valid_permutations: usize = 0;
    let number_of_questions = springs
        .chars()
        .filter(|&c| c == '?')
        .collect::<Vec<char>>()
        .len();
    // println!("Input : {} Pattern : {:?}",line,pattern);
    for i in 0..2usize.pow(number_of_questions as u32) {
        let mut springs = springs.chars().collect::<Vec<char>>().clone();
        let mut counter: usize = 0;

        for idx in 0..springs.len() {
            if springs[idx] == '?' {
                // print!("{}",i >> counter & 1);
                if i >> counter & 1 == 1 {
                    springs[idx] = '#';
                } else {
                    springs[idx] = '.';
                }
                counter += 1;
            }
        }
        let matches = matches_pattern(&springs, &pattern);

        if matches {
            // println!("{i} {:?} {}",springs,matches);
            valid_permutations += 1;
        }
        //println!();
    }
    println!("{} => {}", line, valid_permutations);

    valid_permutations
}

fn solve(input: &str) -> usize {
    let result = input.lines().map(solve_line).sum::<usize>();
    result
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result1: {}", result);
    //let input = include_str!("../../input2.txt");
    //let result = solve(input);
    //println!("Result2: {}",result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line321() {
        let example = "?###???????? 3,2,1";
        assert_eq!(solve_line(example), 10);
    }
    #[test]
    fn example() {
        let example = include_str!("../../example.txt");
        assert_eq!(solve(example), 21);
    }

    #[test]
    fn patterns() {
        let example = ".###.##.#...";
        let pattern = vec![3, 2, 1];
        assert!(matches_pattern(
            &example.chars().collect::<Vec<char>>(),
            &pattern
        ));
        println!("Example 2:");
        let ex2 = vec!['#', '.', '#', '.', '#', '#', '#'];
        let pattern = vec![1, 1, 3];
        assert!(matches_pattern(&ex2, &pattern));
    }
}
