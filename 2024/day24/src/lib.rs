use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
enum LogicGate<'a> {
    And(&'a str, &'a str, &'a str),
    Or(&'a str, &'a str, &'a str),
    Xor(&'a str, &'a str, &'a str),
}

fn parse(input: &str) -> (HashMap<&str, bool>, Vec<LogicGate>) {
    let (const_gates, logic) = input.split_once("\n\n").unwrap();
    let mut logic_gates = Vec::new();
    let mut wires = HashMap::new();
    const_gates.lines().for_each(|line| {
        let (name, state) = line.split_once(": ").unwrap();
        let state = state == "1";
        wires.insert(name, state);
    });

    let reg = Regex::new(r"^(...) (AND|XOR|OR) (...) -> (...)$").unwrap();
    logic.lines().for_each(|line| {
        let captures = reg.captures(line).unwrap();
        let in1 = captures.get(1).unwrap().as_str();
        let gate = captures.get(2).unwrap().as_str();
        let in2 = captures.get(3).unwrap().as_str();
        let out = captures.get(4).unwrap().as_str();
        let logic_gate = match gate {
            "AND" => LogicGate::And(in1, in2, out),
            "XOR" => LogicGate::Xor(in1, in2, out),
            "OR" => LogicGate::Or(in1, in2, out),
            _ => unreachable!("Unknown logic gate type"),
        };
        logic_gates.push(logic_gate);
    });
    (wires, logic_gates)
}

pub fn part1(input: &str) -> String {
    let input = input.trim();

    let (mut wires, logicgates) = parse(input);

    let mut changes: bool = true;
    while changes {
        changes = false;
        for gate in logicgates.iter() {
            match gate {
                LogicGate::And(in1, in2, out) => {
                    if wires.contains_key(in1) && wires.contains_key(in2) {
                        if !wires.contains_key(out) {
                            wires.insert(out, *wires.get(in1).unwrap() && *wires.get(in2).unwrap());
                            changes = true;
                        }
                    }
                }
                LogicGate::Or(in1, in2, out) => {
                    if wires.contains_key(in1) && wires.contains_key(in2) {
                        if !wires.contains_key(out) {
                            wires.insert(out, *wires.get(in1).unwrap() || *wires.get(in2).unwrap());
                            changes = true;
                        }
                    }
                }
                LogicGate::Xor(in1, in2, out) => {
                    if wires.contains_key(in1) && wires.contains_key(in2) {
                        if !wires.contains_key(out) {
                            wires.insert(out, *wires.get(in1).unwrap() ^ *wires.get(in2).unwrap());
                            changes = true;
                        }
                    }
                }
            }
        }
    }

    let mut zs = wires
        .iter()
        .filter(|(&k, &_v)| k.starts_with('z'))
        .collect::<Vec<(&&str, &bool)>>();

    zs.sort_by(|(k, _v), (k2, _v2)| k.cmp(k2));

    let total: i64 = zs
        .iter()
        .enumerate()
        .map(|(idx, (_k, v))| if **v { 2i64.pow(idx as u32) } else { 0 })
        .sum();
    total.to_string()
}

pub fn part2(input: &str) -> String {
    let input = input.trim();

    let (mut wires, logicgates) = parse(input);
    let mut x_in: Vec<&str> = wires
        .iter()
        .filter(|(k, _v)| k.starts_with('x'))
        .map(|(k, _v)| *k)
        .collect();
    x_in.sort();
    let mut y_in: Vec<&str> = wires
        .iter()
        .filter(|(k, _v)| k.starts_with('y'))
        .map(|(k, _v)| *k)
        .collect();
    y_in.sort();

    x_in.iter().for_each(|wire| _ = wires.insert(wire, true));
    y_in.iter().for_each(|wire| _ = wires.insert(wire, false));

    // 45 Bits + 45 Bits = 46 Bits
    let mut changes: bool = true;
    while changes {
        changes = false;
        for gate in logicgates.iter() {
            match gate {
                LogicGate::And(in1, in2, out) => {
                    if wires.contains_key(in1) && wires.contains_key(in2) {
                        if !wires.contains_key(out) {
                            wires.insert(out, *wires.get(in1).unwrap() && *wires.get(in2).unwrap());
                            changes = true;
                        }
                    }
                }
                LogicGate::Or(in1, in2, out) => {
                    if wires.contains_key(in1) && wires.contains_key(in2) {
                        if !wires.contains_key(out) {
                            wires.insert(out, *wires.get(in1).unwrap() || *wires.get(in2).unwrap());
                            changes = true;
                        }
                    }
                }
                LogicGate::Xor(in1, in2, out) => {
                    if wires.contains_key(in1) && wires.contains_key(in2) {
                        if !wires.contains_key(out) {
                            wires.insert(out, *wires.get(in1).unwrap() ^ *wires.get(in2).unwrap());
                            changes = true;
                        }
                    }
                }
            }
        }
    }
    let mut zs = wires
        .iter()
        .filter(|(&k, &_v)| k.starts_with('z'))
        .collect::<Vec<(&&str, &bool)>>();

    zs.sort_by(|(k, _v), (k2, _v2)| k.cmp(k2));
    dbg!(zs);

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
        assert_eq!(result, "4"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, ""); // Replace with the actual expected result
    }
}
