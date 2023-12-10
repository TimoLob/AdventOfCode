use std::collections::HashMap;

use nom::{ sequence::{terminated, separated_pair}, character::complete::alpha1, bytes::complete::tag, IResult};


#[derive(Debug)]
struct Node<'a> {
    left : &'a str,
    right : &'a str
}


impl Node<'_> {
    fn get(&self,inst : &Instruction) -> &str {
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
// PND = (LHJ, NLX)
fn parse_line(line : &str) -> (&str,Node) {
    let result: IResult<&str,&str> = terminated(alpha1, tag(" = ("))(line);
    let (input, current) = result.unwrap();
    let result: IResult<&str, (&str,&str)> = separated_pair(alpha1, tag(", "), terminated(alpha1, tag(")")))(input); 
    let (_,(left,right)) = result.unwrap();
    //println!("Current: {}, Left: {}, Right: {}",current,left,right);
    let node : Node = Node { left: left, right: right };
    return (current,node);
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

    let mut map : HashMap<&str,Node> = HashMap::new();
    for (index,line) in lines.enumerate() {
        println!("{} {}",index,line);
        let (start,node) = parse_line(line);
        map.insert(start, node);
    }

    let mut current_pos = "AAA";
    let mut steps = 0;
    while current_pos!="ZZZ" {
        let instruction = &instructions[steps%instructions.len()];
        println!("Step {}, Current: {}, Instruction: {:?}, Node : {:?}",steps,current_pos,instruction,map[current_pos]);
        
        current_pos = map[current_pos].get(instruction);
        steps += 1;
    }
    return steps;

}

fn main() {
    println!("Hello, world!");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result {}",result);
}



#[cfg(test)]
mod tests {
    use super::*;

   

    #[test]
    fn example1() {
        let example = include_str!("../../example1.txt");
        assert_eq!(solve(example), 2);
    }
    #[test]
    fn example2() {
        let example = include_str!("../../example2.txt");
        assert_eq!(solve(example), 6);
    }
    
}
