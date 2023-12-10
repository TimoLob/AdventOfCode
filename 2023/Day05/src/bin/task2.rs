use std::vec;

use nom::{IResult, multi::fold_many1, character::complete::{self, space0},  sequence::{preceded, terminated}, bytes::complete::tag};

struct Mapping {
    source :  i64,
    dest : i64,
    range: i64,
}


fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    //let (input,seeds) = preceded(tag("seeds: "),separated_list1(space1, digit1))(input)?;
    let (input,numbers) = preceded(tag("seeds: "),
    fold_many1(terminated(complete::i64, space0), 
    Vec::new, 
    |mut acc:Vec<i64>, item| {
        acc.push(item);
        acc
    }))(input)?;
    
    let mut seeds: Vec<i64> = vec![];
    for index in (0..numbers.len()).step_by(2) {
        for seed in numbers[index]..numbers[index]+numbers[index+1] {
            seeds.push(seed);
        }
    }
    return Ok((input,seeds))
}

fn apply_range_map(mapping_vector:&Vec<Mapping>, value : i64) -> i64{
    for mapping in mapping_vector {
        if (mapping.source..mapping.source+mapping.range).contains(&value) {
            return mapping.dest+(value-mapping.source);
        }
    }
    return value;
}

fn solve(input: &str) -> i64 {
    //println!("{}", input);
    let (input, seeds) = parse_seeds(input).expect("Parse seed");
    //println!("Seeds {:?}",seeds);
    println!("{} Seeds",seeds.len());
    let mut mappings: Vec<Vec<Mapping>> = vec![];
    let mut index : usize = 0;
    for line in input.lines() {
        println!("{} Length: {}",line,line.len());
        if line.len()==0 {
            continue;
        }
        else if line.contains("map") {
            mappings.push(vec![]);
            index+=1;
        }
        else {
            let a : Vec<i64> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            let dest = a[0];
            let source = a[1];
            let range = a[2];
            let m = Mapping { source, dest, range };
            mappings[index-1].push(m);

        }
    }

    let lowest = seeds.iter().map(|seed|{
        let mut current = *seed;
        for map in mappings.as_slice() {
            current = apply_range_map(map,current);
        }
        current
    }).min().unwrap();

    return lowest;

}

fn main() {
    //let input = include_str!("../../input.txt");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result: {}", result);
    // Answer: 37806486
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let example = include_str!("../../example.txt");
        assert_eq!(solve(example), 46);
    }
}
