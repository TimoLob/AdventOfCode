use std::collections::{HashMap, VecDeque};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Pulse>),
    Broadcast,
    Button,
}

#[derive(Debug, Clone)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    output: Vec<&'a str>,
}

fn parse_module(input: &str) -> IResult<&str, (&str, Module)> {
    let (input, (mut name, targets)) = separated_pair(
        take_till(|c| c == ' '),
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    )(input)?;
    println!("Module : {} | targets: {:?}", name, targets);
    let module_type = match name.chars().next().expect("First char") {
        '%' => ModuleType::FlipFlop(false),
        '&' => ModuleType::Conjunction(HashMap::new()),
        'b' => ModuleType::Broadcast,
        v => panic!("unexpected char {}", v),
    };

    match module_type {
        ModuleType::FlipFlop(_) | ModuleType::Button | ModuleType::Conjunction(_) => {
            name = &name[1..];
        }
        ModuleType::Broadcast => {}
    };

    Ok((
        input,
        (
            name,
            Module {
                module_type,
                output: targets,
            },
        ),
    ))
}

fn parse(input: &str) -> IResult<&str, HashMap<&str, Module>> {
    let (input, modules) = separated_list1(line_ending, parse_module)(input)?;
    let mut map: HashMap<&str, Module<'_>> = HashMap::new();
    for (name, module) in modules.iter() {
        map.insert(name, module.clone());
    }

    // Iterate over all modules. Conjunction Modules have a Hashmap that contains all their inputs with a Low Pulse
    let mut inputs = HashMap::new(); // Map from Module Receiving input to module sending input
    for (name, module) in modules.iter() {
        for output in module.output.iter() {
            if let Some(output_module) = map.get(output) {
                if let ModuleType::Conjunction(_) = &output_module.module_type {
                    inputs.insert(*output, *name);
                }
            }
        }
    }

    for (receiver, sender) in inputs.iter() {
        if let Some(Module {
            module_type: ModuleType::Conjunction(map),
            ..
        }) = map.get_mut(receiver)
        {
            map.insert(sender, Pulse::Low);
        }
    }

    let button = Module {
        module_type: ModuleType::Button,
        output: vec!["broadcaster"],
    };
    map.insert("button", button);

    Ok((input, map))
}

#[derive(Debug)]
struct Event<'a> {
    module_name: &'a str,
    pulse: Pulse,
    source: &'a str,
}

impl Event<'_> {
    fn new<'a>(module_name: &'a str, pulse: Pulse, source: &'a str) -> Event<'a> {
        Event {
            module_name, // Receiver
            pulse,
            source,
        }
    }
}

fn push_the_button(modules: &mut HashMap<&str, Module>, number_of_times: u64) -> u64 {
    let mut low_pulses: u64 = 0;
    let mut high_pulses: u64 = 0;
    for i in 0..number_of_times {
        // println!("Button Press {}", i);
        let mut event_queue: VecDeque<Event> = VecDeque::new();
        event_queue.push_back(Event::new("button", Pulse::Low, ""));
        while let Some(event) = event_queue.pop_front() {
            //println!("Event: {:?}", event);

            let module = modules.get_mut(event.module_name);
            if module.is_none() {
                // println!("Module {} not found", event.module_name);
                continue;
            }
            let module = module.unwrap();
            let mut output_pulse: Option<Pulse> = None;
            match &module.module_type {
                ModuleType::FlipFlop(state) => {
                    match event.pulse {
                        Pulse::High => {}
                        Pulse::Low => {
                            if *state {
                                module.module_type = ModuleType::FlipFlop(false);
                                output_pulse = Some(Pulse::Low);
                            } else {
                                module.module_type = ModuleType::FlipFlop(true);
                                output_pulse = Some(Pulse::High);
                            }
                        }
                    };
                }
                ModuleType::Conjunction(map) => {
                    let mut new_map = map.clone();

                    new_map.insert(event.source, event.pulse);
                    if new_map.values().all(|&v| v == Pulse::High) {
                        output_pulse = Some(Pulse::Low);
                    } else {
                        output_pulse = Some(Pulse::High);
                    }
                    module.module_type = ModuleType::Conjunction(new_map);
                }
                ModuleType::Broadcast => {
                    output_pulse = Some(event.pulse);
                }
                ModuleType::Button => {
                    output_pulse = Some(event.pulse);
                }
            };
            if let Some(pulse) = output_pulse {
                for output in module.output.iter() {
                    match pulse {
                        Pulse::High => high_pulses += 1,
                        Pulse::Low => low_pulses += 1,
                    }
                    // println!("{} -{:?}-> {} ", event.module_name, pulse, output);
                    event_queue.push_back(Event::new(output, pulse, event.module_name));
                }
            }
        }
    }
    println!("High Pulses: {}", high_pulses);
    println!("Low Pulses: {}", low_pulses);
    low_pulses * high_pulses
}

fn solve(modules: HashMap<&str, Module>) -> u64 {
    push_the_button(&mut modules.clone(), 1000)
}

fn main() {
    let input = include_str!("../../example3.txt");
    let modules = parse(input);
    let result = solve(modules.unwrap().1);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let example = include_str!("../../example.txt");
        let modules = parse(example).unwrap().1;
        assert_eq!(solve(modules), 32000000);
    }

    #[test]
    fn example2() {
        let example = include_str!("../../example2.txt");
        let modules = parse(example).unwrap().1;
        assert_eq!(solve(modules), 11687500);
    }

    #[test]
    fn example3() {
        let example = include_str!("../../example6.txt");
        let modules = parse(example).unwrap().1;
        assert_eq!(solve(modules), 3500 * 4500);
    }
}
