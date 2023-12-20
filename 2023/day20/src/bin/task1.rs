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

    // Iterate over all modules. Find modules that output to Conjunction Modules.
    // Conjunction moduls have a Hashmap that contains all their inputs with initially Low Pulse
    let mut inputs = HashMap::new(); // Map Receiver : Sender
    for (name, module) in modules.iter() {
        for output in module.output.iter() {
            if let Some(output_module) = map.get(output) {
                if let ModuleType::Conjunction(_) = &output_module.module_type {
                    inputs.insert(*output, *name);
                }
            }
        }
    }
    // Initialize input map for conjunction modules
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
    update_only: bool,
}

impl Event<'_> {
    fn new<'a>(
        module_name: &'a str,
        pulse: Pulse,
        source: &'a str,
        update_only: bool,
    ) -> Event<'a> {
        Event {
            module_name, // Receiver
            pulse,
            source,
            update_only,
        }
    }
}

fn push_the_button(modules: &mut HashMap<&str, Module>, number_of_times: u64) -> u64 {
    let mut low_pulses: u64 = 0;
    let mut high_pulses: u64 = 0;
    for i in 0..number_of_times {
        // println!("Button Press {}", i);

        // Event Queue. Contains events that need to be processed in order.
        // Initialized with a button press event
        let mut event_queue: VecDeque<Event> = VecDeque::new();
        event_queue.push_back(Event::new("button", Pulse::Low, "", false));
        while let Some(event) = event_queue.pop_front() {
            let module = modules.get_mut(event.module_name);
            if module.is_none() {
                // println!("Module {} not found", event.module_name);
                // Some Modules send pulses to modules that are not in the list.
                continue;
            }
            let module = module.unwrap();
            if event.update_only {
                // If the event is an update event, update the module, if it is a Conjunction, and continue
                if let ModuleType::Conjunction(map) = &module.module_type {
                    let mut new_map = map.clone();

                    new_map.insert(event.source, event.pulse); // First update the map

                    // Update the module with the new map
                    module.module_type = ModuleType::Conjunction(new_map);
                }

                continue;
            }

            let mut output_pulse: Option<Pulse> = None;
            match &module.module_type {
                ModuleType::FlipFlop(state) => {
                    match event.pulse {
                        Pulse::High => {} // Do nothing
                        Pulse::Low => {
                            // On Low : Flip state and send corresponding Pulse
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
                    // Ugly code to update the map, but it should work.

                    if map.values().all(|&v| v == Pulse::High) {
                        // Then if all values are high, send low pulse
                        output_pulse = Some(Pulse::Low);
                    } else {
                        output_pulse = Some(Pulse::High); // Otherwise send high pulse
                    }
                }
                ModuleType::Broadcast => {
                    // Broadcast module sends the same pulse it receives
                    output_pulse = Some(event.pulse);
                }
                ModuleType::Button => {
                    // Button module sends a low pulse (It always receives a low pulse)
                    output_pulse = Some(event.pulse);
                }
            };
            // If the module sends a pulse, add the pulse to the event queue
            // High and low pulses are counted as they are added to the event queue
            if let Some(pulse) = output_pulse {
                for output in module.output.iter() {
                    match pulse {
                        Pulse::High => high_pulses += 1,
                        Pulse::Low => low_pulses += 1,
                    }
                    event_queue.push_front(Event::new(output, pulse, event.module_name, true));
                    event_queue.push_back(Event::new(output, pulse, event.module_name, false));
                }
            }
        }
    }
    println!("High Pulses: {}", high_pulses);
    println!("Low Pulses: {}", low_pulses);
    low_pulses * high_pulses
}

fn solve(modules: HashMap<&str, Module>) -> u64 {
    // Press the button 1000 times
    push_the_button(&mut modules.clone(), 1000)
}

fn main() {
    let input = include_str!("../../input.txt");
    let modules = parse(input);
    let result = solve(modules.unwrap().1);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
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
}
