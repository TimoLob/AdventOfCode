use core::num;

use ndarray::{Array, Array2, array};
use nom::{IResult, Parser, branch::alt, bytes::complete::tag, character::complete::{self, line_ending, space1}, combinator::value, multi::{many1, separated_list1}, sequence::{preceded, separated_pair}};

type PresentShape = Array2<bool>;
#[derive(Debug)]
struct Tree {
    gridsize : Gridsize,
    presents : Vec<usize>
}
#[derive(Debug)]
struct Gridsize {
    x : usize,
    y : usize
}

fn parse_present(input: &str) -> IResult<&str, PresentShape> {
    let (input, lines) = preceded(preceded(complete::i32, tag(":\n")), 
    separated_list1(line_ending, many1(alt((
        value(true, tag("#")),
        value(false, tag("."))
    ))))).parse(input)?;
    let flattened = lines.iter().flat_map(|v| v.iter()).cloned().collect::<Vec<bool>>();
    let arr : Array2<bool>= Array::from_shape_vec((3,3),flattened).unwrap();
    // println!("{:?}",arr);

    return Ok((input,arr))
}


fn parse_presents(input: &str) -> IResult<&str, Vec<PresentShape>> {
    separated_list1(tag("\n\n"), parse_present).parse(input)
}

fn parse_tree(input : &str) -> IResult<&str, Tree> {
    let (input, (size, presents)) = separated_pair(separated_pair(complete::usize, tag("x"), complete::usize), tag(": "), separated_list1(space1, complete::usize)).parse(input)?;
    Ok((input, Tree {
        gridsize: Gridsize { x: size.0, y: size.1 },
        presents : presents
    }))
}



fn parse(input: &str) -> IResult<&str, (Vec<PresentShape>, Vec<Tree>)> {
    separated_pair(parse_presents, tag("\n\n"), separated_list1(line_ending, parse_tree)).parse(input)
}


pub fn part1(input: &str) -> String {
    let input = input.trim();
    let (input, (shapes, trees)) = parse(input).unwrap();
    //println!("{:?},{:?},{:?}",input,shapes,trees);
    let possible = trees.iter().filter(|tree| {
        // Check if even possible by grid size
        let num_presents = tree.presents.iter().sum::<usize>();
        let spaces_x = tree.gridsize.x / 3;
        let spaces_y = tree.gridsize.y / 3;
        if num_presents > spaces_x * spaces_y {
            return false;
        }
        true
    }).count();
    possible.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "2"); 
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, ""); // Replace with the actual expected result
    }
}
