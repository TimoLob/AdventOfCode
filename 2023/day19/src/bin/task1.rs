use std::collections::HashMap;

#[derive(Debug)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
enum Rule {
    // Target, <>, value to compare to, variable to compare
    Condition(String, Operator, i64, char),
    Default(String),
}

#[derive(Debug)]
struct Part {
    ratings: HashMap<char, i64>,
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

        Rule::Condition(target.to_string(), comparator, value, variable)
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
                //println!("{:?}", key);
                //println!("{:?}", value);
                map.insert(key, value);
            }
            Part { ratings: map }
        })
        .collect();
    parts
}

fn solve(input: &str) -> i64 {
    let splits = input.split("\n\n").collect::<Vec<&str>>();
    //println!("{:?}", splits);
    let workflow_s = splits[0];
    let parts_s = splits[1];
    let workflows = parse_workflow(workflow_s);
    let parts = parse_parts(parts_s);
    let mut total = 0;
    for part in parts {
        println!("{:?}", part);
        let mut current_workflow = "in";
        loop {
            println!(
                "Current Workflow: {:?} | Current Part : {:?}",
                current_workflow, part
            );
            let rules = workflows
                .get(current_workflow)
                .expect("Expect workflow to exist");
            for rule in rules {
                //Target, <>, value to compare to, variable to compare
                match rule {
                    Rule::Condition(target, comparator, cvalue, variable) => {
                        let value = *part
                            .ratings
                            .get(variable)
                            .expect("Part should have rating for var");
                        let matches = match comparator {
                            Operator::LessThan => value < *cvalue,
                            Operator::GreaterThan => value > *cvalue,
                        };
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
                println!("Accepted part {:?} with total {}", part, part_total);
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
