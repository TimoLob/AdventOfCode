use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: i64,
    max: i64,
}

#[derive(Debug)]
enum Rule {
    // Target, Range of allowed value, variable to compare
    Condition(String, Range, char),
    Default(String),
}

#[derive(Debug)]
struct Part {
    ratings: HashMap<char, Range>,
}

impl Range {
    fn new(min: i64, max: i64) -> Range {
        Range { min, max }
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        if self.max < other.min || other.max < self.min {
            None
        } else {
            Some(Range {
                min: self.min.max(other.min),
                max: self.max.min(other.max),
            })
        }
    }

    fn inverse(&self) -> Vec<Range> {
        let mut ranges = vec![];
        if self.min > 1 {
            ranges.push(Range::new(1, self.min - 1));
        }
        if self.max < 4000 {
            ranges.push(Range::new(self.max + 1, 4000));
        }
        ranges
    }

    fn contains(&self, value: i64) -> bool {
        self.min <= value && value <= self.max
    }
}

fn parse_workflow(input: &str) -> HashMap<&str, Vec<Rule>> {
    let mut workflows = HashMap::new();
    for line in input.lines() {
        let split = line.split('{').collect::<Vec<&str>>();
        let name = split[0];
        let rules_s = split[1].split(',');
        //println!("{:?}", name);

        let rules = rules_s.map(parse_rule).collect::<Vec<Rule>>();
        workflows.insert(name, rules);
    }

    workflows
}

// a<2006:qkq m>2090:A rfg}
fn parse_rule(rule_s: &str) -> Rule {
    if rule_s.contains(':') {
        let split = rule_s.split(':').collect::<Vec<&str>>();
        let condition = split[0];
        let target = split[1];
        //println!("{:?}", condition);
        //println!("{:?}", target);
        // Conver a<2006 to a closure a can be one of x,m,a,s. The comparison operator can be <,>
        let variable = condition.chars().next().unwrap();
        let operator = condition.chars().nth(1).unwrap();
        let value = condition[2..].parse::<i64>().unwrap();
        let comparator = match operator {
            '<' => Operator::LessThan,
            '>' => Operator::GreaterThan,
            _ => panic!("Invalid operator"),
        };

        let range = match comparator {
            Operator::LessThan => Range::new(1, value - 1),
            Operator::GreaterThan => Range::new(value + 1, 4000),
        };
        Rule::Condition(target.to_string(), range, variable)
    } else {
        let target = rule_s.replace('}', "");
        Rule::Default(target)
    }
}

fn solve(input: &str) -> i64 {
    let splits = input.split("\n\n").collect::<Vec<&str>>();
    //println!("{:?}", splits);
    let workflow_s = splits[0];
    let parts_s = splits[1];
    let workflows = parse_workflow(workflow_s);
    let mut parts = VecDeque::new();
    let mut map = HashMap::new();
    map.insert('x', Range::new(1, 4000));
    map.insert('m', Range::new(1, 4000));
    map.insert('a', Range::new(1, 4000));
    map.insert('s', Range::new(1, 4000));

    // parts.push_back(("in", Part { ratings: map }));
    map.insert('x', Range::new(1, 4000));
    map.insert('m', Range::new(1801, 4000));
    map.insert('a', Range::new(1, 4000));
    map.insert('s', Range::new(1, 2770));
    parts.push_back(("hdj", Part { ratings: map }));
    let mut acceped_parts: Vec<Part> = vec![];
    let mut rejected_parts: Vec<Part> = vec![];
    while let Some((workflow_name, part)) = parts.pop_front() {
        let workflow = workflows.get(workflow_name).expect("Workflow should exist");
        let mut new_parts: VecDeque<(&str, Part)> = VecDeque::new();
        for rule in workflow.iter() {
            match rule {
                Rule::Condition(target, range, var) => {
                    let value = part.ratings.get(var).unwrap();

                    if let Some(intersection) = value.intersection(range) {
                        let ranges = intersection.inverse();
                        for range in ranges {
                            let mut new_part = Part {
                                ratings: part.ratings.clone(),
                            };
                            new_part.ratings.insert(*var, range);
                            new_parts.push_back((workflow_name, new_part));
                        }
                        let mut new_part = Part {
                            ratings: part.ratings.clone(),
                        };
                        new_part.ratings.insert(*var, intersection);
                        new_parts.push_back((target, new_part));
                        break;
                    }
                }
                Rule::Default(target) => {
                    let new_part = Part {
                        ratings: part.ratings.clone(),
                    };
                    new_parts.push_back((target, new_part));
                    break;
                }
            };
        }
        for (workflow, new_part) in new_parts {
            if workflow == "A" {
                acceped_parts.push(new_part);
            } else if workflow == "R" {
                rejected_parts.push(new_part);
            } else {
                parts.push_back((workflow, new_part));
            }
        }
    }
    println!("{:?}", acceped_parts);
    acceped_parts.len() as i64
}

fn main() {
    let input = include_str!("../../example.txt");
    let result = solve(input);
    println!("Result: {:?}", result);
}
