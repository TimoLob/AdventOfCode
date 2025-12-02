


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
    let num_digits = digit_count(n);
    let num_str = n.to_string();
    //println!("---{}---({})",n,num_digits);
    for chunk_size in 1..(num_digits/2+1) {
        //println!("Chunk size {}",chunk_size);
        let chars = num_str.chars().collect::<Vec<char>>();
        let exp = 10u64.pow(chunk_size);
        let lower = n%exp;
        //println!("lower : {}",lower);
        let all_match =chars.chunks(chunk_size as usize).all(|chunk| { 
            let s = String::from_iter(chunk);
            let num = s.parse::<u64>().expect("Failed to parse chunk");
            //println!("num : {}",num);
            num == lower
        });
        if all_match {
            //println!("Repeating");
            return true
        }
    }
    //println!("Not Repeating");
    false

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

    #[test]
    fn is_repeating() {
        // Twice
        assert!(is_num_repeating(11));
        assert!(is_num_repeating(1010));
        assert!(is_num_repeating(1188511885));
        assert!(is_num_repeating(446446));
        assert!(is_num_repeating(38593859));
        // Thrice
        assert!(is_num_repeating(111));
        assert!(is_num_repeating(565656));
        assert!(is_num_repeating(824824824));
        // 5 times
        assert!(is_num_repeating(2121212121));

        // Not 
        assert!(!is_num_repeating(95));
        assert!(!is_num_repeating(101));
        assert!(!is_num_repeating(38593862));
    }

}
