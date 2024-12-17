use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
enum OP {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct Registers {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Debug)]
enum SideEffect {
    None,
    Jump(usize),
    Print(i32),
}

impl OP {
    fn from_opcode(opcode: i32) -> Self {
        match opcode {
            0 => OP::Adv,
            1 => OP::Bxl,
            2 => OP::Bst,
            3 => OP::Jnz,
            4 => OP::Bxc,
            5 => OP::Out,
            6 => OP::Bdv,
            7 => OP::Cdv,
            _ => unreachable!("Unexpected opcode"),
        }
    }

    fn perform_op(&self, operand: i32, registers: &mut Registers) -> SideEffect {
        let combo_operand = match operand {
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            7 => unreachable!("Operand 7 should not appear in valid programs"),
            o if (0..=3).contains(&o) => o,
            u => unreachable!("Unknown op code {}", u),
        };
        match self {
            OP::Adv => {
                let numerator = registers.a;
                let denominator = 2i32.pow(combo_operand as u32);
                registers.a = numerator / denominator;
            }
            OP::Bxl => {
                registers.b = registers.b ^ operand; // Literal operand
            }
            OP::Bst => {
                registers.b = combo_operand % 8;
            }
            OP::Jnz => {
                if registers.a != 0 {
                    return SideEffect::Jump(operand as usize);
                }
            }
            OP::Bxc => {
                registers.b = registers.b ^ registers.c;
            }
            OP::Out => return SideEffect::Print(combo_operand % 8),
            OP::Bdv => {
                let numerator = registers.a;
                let denominator = 2i32.pow(combo_operand as u32);
                registers.b = numerator / denominator;
            }
            OP::Cdv => {
                let numerator = registers.a;
                let denominator = 2i32.pow(combo_operand as u32);
                registers.c = numerator / denominator;
            }
        };
        SideEffect::None
    }
}

fn parse(input: &str) -> (Registers, Vec<i32>) {
    let (register_input, program_input) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r"^Register (.): (\d+)$").unwrap();
    let mut registers: HashMap<char, i32> = HashMap::new();
    register_input.lines().for_each(|line| {
        let captures = re.captures(line).unwrap();
        let reg = captures.get(1).unwrap().as_str().chars().next().unwrap();
        let val = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        registers.insert(reg, val);
    });

    let (_, program_input) = program_input.split_once(' ').unwrap();
    let program = program_input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let a = *registers.get(&'A').unwrap();
    let b = *registers.get(&'B').unwrap();
    let c = *registers.get(&'C').unwrap();
    let registers = Registers { a, b, c };
    (registers, program)
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let (mut registers, program) = parse(input);
    let mut ip = 0;
    let mut output: Vec<i32> = Vec::new();
    loop {
        if ip > program.len() - 2 {
            break;
        }
        let opcode = program[ip];
        let operand = program[ip + 1];
        let op = OP::from_opcode(opcode);
        let side_effect = op.perform_op(operand, &mut registers);
        match side_effect {
            SideEffect::None => ip += 2,
            SideEffect::Jump(addr) => ip = addr,
            SideEffect::Print(val) => {
                output.push(val);
                ip += 2
            }
        };
    }
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0"); // Replace with the actual expected result
    }
}
