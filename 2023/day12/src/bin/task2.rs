

fn number_of_valid_permutations(line : &[char], pattern: &[usize]) -> usize {
    
    if pattern.len() == 0 {
        if !line.contains(&'#') {
            return 1;
        }
        return 0;
    }
    
    if line.len() == 0 {
        return 0;
    }

    let first_char = *line.first().unwrap();
    if  first_char == '.'{
        return number_of_valid_permutations(&line[1..], pattern);
    }
    else if first_char == '#' {

        let number_of_defects = *pattern.first().expect("Should have first element");
        if line.starts_with(&['#'].repeat(number_of_defects)) {
            return number_of_valid_permutations(&line[number_of_defects..],&pattern[1..]);
        }
        return 0;
    }
    else if first_char == '?' {
        let mut permuations = 0;
    }
    return 0;
}


// ?###???????? 3,2,1
fn solve_line(line : &str) -> usize {
    let split = line.split(" ").collect::<Vec<&str>>();
    let springs = split[0].to_string();
    let pattern_string = split[1];
    let pattern = pattern_string.split(",").map(|s| s.parse::<usize>().expect("Should be number")).collect::<Vec<usize>>();
    let springs = springs.chars().collect::<Vec<char>>();
    return number_of_valid_permutations(&springs, &pattern);
    
}


fn solve(input :&str) -> usize{
    let result = input.lines().map(solve_line).sum::<usize>();
    return result;
}

fn main() {

    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result1: {}",result);
    let input = include_str!("../../input2.txt");
    let result = solve(input);
    println!("Result2: {}",result);
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
        let pattern = vec![3,2,1];
        println!("Example 2:");
        let ex2 = vec!['#', '.', '#', '.', '#', '#', '#'];
        let pattern = vec![1, 1, 3];
    }
    
}