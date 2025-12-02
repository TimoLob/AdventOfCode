use fancy_regex::Regex;



fn digit_count(n: u64) -> u32 {
    if n==0 {
        1
    }else {
        n.ilog10()+1
    }
}

fn is_num_twice(n: u64) -> bool {
    let num_digits = digit_count(n);
    if num_digits % 2 != 0 {
        return false;
    }
    let exp = 10u64.pow(num_digits/2);
    let upper = n/exp;
    let lower = n%exp;
    //println!("{} : {} | {}",n,upper,lower);
    upper==lower
}

fn is_num_repeating(n:u64) -> bool {
    let s = n.to_string();
    let re = Regex::new(r"^(\d+)\1+$").expect("Invalid regex");
    re.is_match(&s).expect("Regex error")
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let ranges = input.split(",").map(|range| {
        let (first,second) = range.split_once("-").unwrap();
        first.parse::<u64>().unwrap()..(second.parse::<u64>().unwrap()+1)
    });

    let sames = ranges.map(|range| {
        range.filter(|number| {
                is_num_twice(*number)

        }).sum::<u64>()
    }).sum::<u64>();
    sames.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let ranges = input.split(",").map(|range| {
        let (first,second) = range.split_once("-").unwrap();
        first.parse::<u64>().unwrap()..(second.parse::<u64>().unwrap()+1)
    });
    let sames = ranges.map(|range| {
        range.filter(|number| {
                is_num_repeating(*number)

        }).sum::<u64>()
    }).sum::<u64>();
    sames.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "1227775554"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "4174379265"); // Replace with the actual expected result
    }
    #[test] 
    fn count_digits() {
        for i in 0..10 {
            assert_eq!(1,digit_count(i));
        }
        for i in 10..100 {
            assert_eq!(2,digit_count(i));

        }
        for i in 100..1000 {
            assert_eq!(3,digit_count(i));

        }
        
    }

    #[test]
    fn is_twice() {
        assert!(is_num_twice(11));
        assert!(is_num_twice(1010));
        assert!(is_num_twice(1188511885));
        assert!(is_num_twice(446446));
        assert!(is_num_twice(38593859));
        // Not 
        assert!(!is_num_twice(95));
        assert!(!is_num_twice(101));
        assert!(!is_num_twice(38593862));
    }

}
