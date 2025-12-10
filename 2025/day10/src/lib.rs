
use std::collections::VecDeque;

use nom::{IResult, Parser, branch::alt, bytes::complete::tag, character::complete::{self, char, line_ending, space0, space1}, combinator::value, multi::{many1, separated_list1}, sequence::delimited};
use z3::{Optimize, ast::Int};


type Button = Vec<usize>;
type Joltage = usize;

#[derive(Debug)]
struct Line {
    target: Vec<bool>,
    buttons : Vec<Button>,
    joltage_req : Vec<Joltage>,
}

impl Line {
    fn new(target: Vec<bool>,buttons:Vec<Vec<usize>>,joltage_req:Vec<usize>) -> Self {
        Line { target, buttons, joltage_req }
    }
    
}

fn parse_lights(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(tag("["), many1(alt((
        value(true, char('#')),
        value(false, char('.')),
    ))), tag("]")).parse(input)
}

fn parse_button(input: &str) -> IResult<&str, Vec<Button>> {
    delimited(space0, separated_list1(space1, 
        delimited(tag("("), separated_list1(tag(","), complete::usize), tag(")"))
    ), space0).parse(input)
}

fn parse_joltage_requirement(input: &str) -> IResult<&str, Vec<Joltage>> {
    delimited(tag("{"), separated_list1(tag(","), complete::usize), tag("}")).parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(line_ending, 
        (parse_lights,parse_button,parse_joltage_requirement)
        .map(|(a,b,c)| Line::new(a,b,c))
    ).parse(input)
} 

struct State {
    steps: usize,
    lights : Vec<bool>
}

impl State {
    fn from_button_press(state: &Self, b:&Button) -> Self {
        let mut new_lights = state.lights.clone();
        b.iter().for_each(|&p| new_lights[p] = !new_lights[p]);

        State { steps: state.steps+1, lights: new_lights }
    }
}

fn solve_line(line:&Line) -> usize {

    let mut queue :VecDeque<State> = VecDeque::new();
    queue.push_back(State { steps: 0, lights: vec![false;line.target.len()] });
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for button in line.buttons.iter() {
            let new_state = State::from_button_press(&current, button);
            if new_state.lights == line.target {
                return new_state.steps;
            }
            queue.push_back(new_state);
        }
    }
    panic!()
}




fn solve_line_2(line:&Line) -> usize {

    let solver = Optimize::new();
    let mut num_button_presses: Vec<Int> = Vec::new();
    for i in 0..line.buttons.len() {
        let button = Int::fresh_const(&format!("B{}",i));
        solver.assert(&button.ge(0));
        num_button_presses.push(button);
    }
    for (idx, joltage) in line.joltage_req.iter().enumerate() {
        let relevant_button_idx = 
        line.buttons.iter().enumerate()
            .filter(|(_,b)| {
                b.contains(&idx)
            });
        let relevant_buttons = relevant_button_idx.map(|(i,_)| num_button_presses[i].clone()).collect::<Vec<Int>>();
        let total = Int::add(&relevant_buttons);
            solver.assert(&total.eq(*joltage as i64));

    }
    solver.minimize(&Int::add(&num_button_presses));
    solver.check(&[]);
    //println!("{:?}",solver);
    let model = solver.get_model().unwrap();
    let r:u64 = num_button_presses.iter().map(|var| model.eval(var, true).unwrap().as_u64().unwrap()).sum();

    r as usize
}


pub fn part1(input: &str) -> String {
    let input = input.trim();
    let (_input, lines) = parse(input).unwrap();
    let total = lines.iter().map(|line| {
        solve_line(line)
    }).sum::<usize>();
    total.to_string()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let (_input, lines) = parse(input).unwrap();
    let total = lines.iter().map(|line| {
        solve_line_2(line)
    }).sum::<usize>();
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "7"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "33"); // Replace with the actual expected result
    }
}
