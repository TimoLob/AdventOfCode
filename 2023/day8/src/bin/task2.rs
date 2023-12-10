use std::collections::HashMap;

use nom::{ sequence::{terminated, separated_pair}, character::complete::alpha1, bytes::complete::{tag, take}, IResult};
use num::integer::lcm;

fn find_lcm(numbers: &Vec<usize>) -> Option<usize> {
    if numbers.is_empty() {
        return None;
    }

    let mut lcm_result = numbers[0];

    for &num in numbers.iter().skip(1) {
        lcm_result = lcm(lcm_result, num);
    }

    Some(lcm_result)
}

#[derive(Debug)]
struct Node {
    name : String,
    left :  usize,
    right : usize,
    terminal : bool
}


impl Node {
    fn get(&self,inst : &Instruction) -> usize {
        match inst {
            Instruction::Left => self.left,
            Instruction::Right => self.right
        }
    }
    
}
#[derive(Debug)]
enum Instruction {
    Left=0,
    Right=1
}

fn solve_one(instructions: &Vec<Instruction>,nodes : &Vec<Node>, start_index : usize) -> usize{
    let mut current_pos = start_index;
    let mut steps = 0;
    while !nodes[current_pos].terminal {
        let instruction = &instructions[steps%instructions.len()];
        //println!("Step {}, Current: {}, Instruction: {:?}, Node : {:?}",steps,current_pos,instruction,map[current_pos]);
        
        current_pos = nodes[current_pos].get(instruction);
        steps += 1;
    }
    return steps;
}

fn solve(input : &str) -> usize {
    
    
    let mut lines = input.lines();
    let first_line = lines.next().expect("First Line");

    let instructions : Vec<Instruction> = first_line.chars().map(|c| {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            value => panic!("Unexpected character :{}:", value)
        }
    }).collect();

    let mut map : HashMap<&str,usize> = HashMap::new(); // Maps name of node to index
    let mut nodes : Vec<Node> = vec![];
    let mut current_nodes: Vec<usize>  = vec![];
    for line in lines {
        let result: IResult<&str,&str> = terminated(take(3usize), tag(" = ("))(line);
        let (input, current) = result.unwrap();
        let node = Node { name: current.to_string(), left: 0, right: 0, terminal: current.ends_with("Z") };
        nodes.push(node);
        let index = nodes.len()-1; 
        if current.ends_with("A") {
            current_nodes.push(index);
        }
        map.insert(current, index);
    }

    let mut lines = input.lines();
    let _ = lines.next().expect("First Line");

    for line in lines {
        let result: IResult<&str,&str> = terminated(take(3usize), tag(" = ("))(line);
        let (input, current) = result.unwrap();
        let result: IResult<&str, (&str,&str)> = separated_pair(take(3usize), tag(", "), terminated(take(3usize), tag(")")))(input); 
        let (_,(left,right)) = result.unwrap();
        let left_index = *map.get(left).unwrap();
        let right_index = *map.get(right).unwrap();
        let current_index = *map.get(current).unwrap();
        nodes[current_index].left = left_index;
        nodes[current_index].right = right_index;

    }

    println!("{:?}",nodes);

    let mut steps: Vec<usize> = current_nodes.iter().map(|x| {
        solve_one(&instructions, &nodes, *x)
    }).collect();

    return find_lcm(&steps).unwrap();

}

fn main() {
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result {}",result);
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example3() {
        let example = include_str!("../../example3.txt");
        assert_eq!(solve(example), 6);
    }
    
}
