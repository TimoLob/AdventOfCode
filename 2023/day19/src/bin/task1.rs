use std::collections::HashMap;

#[derive(Debug)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
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
    ratings: HashMap<char, i64>,
}

impl Range {
    fn new(min: i64, max: i64) -> Range {
        Range { min, max }
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

// {x=787,m=2655,a=1222,s=2876}
fn parse_parts(input: &str) -> Vec<Part> {
    let parts = input
        .lines()
        .map(|line| {
            let line = line.replace(['{', '}'], "");
            let ratings = line.split(',');
            let mut map = HashMap::new();
            for rating in ratings {
                let split = rating.split('=').collect::<Vec<&str>>();
                let key = split[0].chars().next().unwrap();
                let value = split[1].parse::<i64>().unwrap();

                map.insert(key, value);
            }
            Part { ratings: map }
        })
        .collect();
    parts
}

fn solve(input: &str) -> i64 {
    let splits = input.split("\n\n").collect::<Vec<&str>>();
    let workflow_s = splits[0];
    let parts_s = splits[1];
    let workflows = parse_workflow(workflow_s);
    let parts = parse_parts(parts_s);
    let mut total = 0;
    for part in parts {
        // println!("{:?}", part);
        let mut current_workflow = "in";
        loop {
            let rules = workflows
                .get(current_workflow)
                .expect("Expect workflow to exist");
            for rule in rules {
                //Target, <>, value to compare to, variable to compare
                match rule {
                    Rule::Condition(target, range, variable) => {
                        let value = *part
                            .ratings
                            .get(variable)
                            .expect("Part should have rating for var");
                        let matches = range.contains(value);
                        if matches {
                            current_workflow = target;
                            break;
                        }
                    }
                    Rule::Default(target) => {
                        current_workflow = target;
                        break;
                    }
                };
            }
            if current_workflow == "A" {
                let part_total = part.ratings.iter().fold(0, |acc, (_, value)| acc + *value);
                total += part_total;
                // println!("Accepted part {:?} with total {}", part, part_total);
                break;
            } else if current_workflow == "R" {
                break;
            }
        }
    }
    total
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result: {:?}", result);
}
