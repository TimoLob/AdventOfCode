
use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use nom::character::complete::{self, newline, space1, space0};
use nom::multi::separated_list1;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mult 
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64).parse(input)
}

fn parse_operators(input: &str) -> IResult<&str, Vec<Operator>> {
    separated_list1(space1, tag("+").or(tag("*")).map(|c| 
    match c {
        "+" => {Operator::Add},
        "*" => {Operator::Mult},
        _ => panic!()
    }
    )).parse(input)
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let (_input, (numbers, ops)) = separated_pair(separated_list1(space0.and(newline).and(space0), parse_numbers), newline, parse_operators).parse(input).unwrap();
    let len = ops.len();
    debug_assert!(numbers.iter().all(|v| v.len()==len));

    let total = (0..len).map(|i| {
        let op = ops[i];
        let init = match op {
            Operator::Add => 0,
            Operator::Mult => 1,
        };
        numbers.iter().fold(init, |acc,x| {
            match op {
                Operator::Add => acc+x[i],
                Operator::Mult => acc*x[i],
            }
        })
    }).sum::<u64>();
    total.to_string()
}

fn parse_operators_count_space(input: &str) -> (Vec<Operator>, Vec<usize>) {
    let chars = input.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut ops = Vec::new();
    let mut spaces = Vec::new();
    while i < chars.len() {

        match chars[i] {
            '+' => ops.push(Operator::Add),
            '*' => ops.push(Operator::Mult),
            _ => panic!()
        }
        i+=1;
        let mut counting = 0;
        while i < chars.len() && chars[i] ==' ' {
            counting+=1;
            i+=1;
        }
        if i >= chars.len() {
            counting +=1;
        }
        spaces.push(counting);
    }

    return (ops,spaces)
    
}


fn part2_parse(input: &str) -> (Vec<Vec<u64>>, Vec<Operator>, Vec<usize>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let num_lines = lines.len();
    let (ops, spaces) = parse_operators_count_space(lines[num_lines-1]);
    //println!("{:?}, {:?}",ops,spaces);
    let numbers = (0..(num_lines-1)).map(|i| {
        lines[i].chars().map(|c| {
            if c.is_digit(10) {
                c.to_digit(10).unwrap() as u64
            }
            else {
                0
            }
        })
    }.collect::<Vec<u64>>()).collect();
    return (numbers, ops, spaces)
    
}

fn build_number(offset:usize, numbers: &Vec<Vec<u64>>) -> u64 {
    let mut total = 0;
    for v in numbers.iter() {
        let d = v[offset];
        if d != 0 {
            total = total*10 + d;
        }
    }
    total
}

pub fn part2(input: &str) -> String {
    let (numbers, ops, spaces) = part2_parse(input);
    //println!("{:?}", numbers);
    let mut offset =0;
    let mut result = 0;
    for i in 0..ops.len() {
        let op = ops[i];
        let init = match op {
            Operator::Add => 0,
            Operator::Mult => 1,
        };
        let operands = (0..spaces[i]).map(|j| build_number(offset+j, &numbers));
        let sum = operands.fold(init, |acc,x| {
            match op {
                Operator::Add => acc+x,
                Operator::Mult => acc*x,
            }
        });

        offset+=spaces[i] + 1;
        //println!("={}",sum);
        result += sum;
        
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "4277556"); 
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "3263827"); 
    }
}
