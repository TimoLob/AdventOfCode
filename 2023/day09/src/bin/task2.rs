use nom::{IResult, multi::{fold_many1, separated_list1}, character::complete::{line_ending, space0}, sequence::terminated};


fn parse_line(input:&str) -> IResult<&str,Vec<i64>> {
    let (input, mut seq) = fold_many1(terminated(nom::character::complete::i64, space0), Vec::new, |mut acc,item| {
        acc.push(item);
        acc
    })(input)?;
    seq.reverse(); // Literally the only change
    return Ok((input,seq));
}

fn parse(input:&str) -> IResult<&str,Vec<Vec<i64>>> {
    let (input, sequences) = separated_list1(line_ending, parse_line)(input)?;
    assert!(input=="");
    return Ok((input,sequences));
}

fn next_in_sequence(sequence : &Vec<i64>) -> i64 {
    if sequence.iter().all(|&x| x==0) {
        return 0;
    }
    let derivative : Vec<i64> = sequence.windows(2).map(|pair| {
        pair[1]-pair[0]
    }).collect();
    
    let next = next_in_sequence(&derivative)+sequence.last().unwrap();
    println!("{:?} , {}",sequence,next);
    return next;
}


fn solve(input: &str) -> i64{
    let (_,sequences) = parse(input).expect("Should parse");
    println!("{:?}",sequences);

    let next_elements = sequences.iter().map(|seq| next_in_sequence(seq));

    return next_elements.sum::<i64>();
}

fn main() {
    println!("Hello, world!");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result: {}",result);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let example = include_str!("../../example.txt");
        assert_eq!(solve(example), 114);
    }
    
}
