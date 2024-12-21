use core::panic;
use std::collections::HashMap;

use glam::IVec2;
use pathfinding::prelude::dijkstra;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Key {
    Numeric(usize),
    Directional(Direction),
    A,
}

impl Direction {
    fn to_ivec(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::X,
            Direction::Right => IVec2::NEG_X,
        }
    }
}

#[derive(Clone, Debug)]
struct Keypad {
    keys: HashMap<Key, IVec2>,
    positions: HashMap<IVec2, Key>,
}
impl Keypad {
    fn new_numeric() -> Self {
        let mut keys = HashMap::new();
        let mut positions = HashMap::new();
        let key_vec = vec![
            (Key::Numeric(7), IVec2 { x: 0, y: 0 }),
            (Key::Numeric(8), IVec2 { x: 1, y: 0 }),
            (Key::Numeric(9), IVec2 { x: 2, y: 0 }),
            (Key::Numeric(4), IVec2 { x: 0, y: 1 }),
            (Key::Numeric(5), IVec2 { x: 1, y: 1 }),
            (Key::Numeric(6), IVec2 { x: 2, y: 1 }),
            (Key::Numeric(1), IVec2 { x: 0, y: 2 }),
            (Key::Numeric(2), IVec2 { x: 1, y: 2 }),
            (Key::Numeric(3), IVec2 { x: 2, y: 1 }),
            (Key::Numeric(0), IVec2 { x: 1, y: 2 }),
            (Key::A, IVec2 { x: 2, y: 2 }),
        ];
        key_vec.iter().for_each(|(key, pos)| {
            _ = keys.insert(*key, *pos);
            _ = positions.insert(*pos, *key);
        });

        Keypad { keys, positions }
    }

    fn new_directional() -> Self {
        let mut keys = HashMap::new();

        let mut positions = HashMap::new();
        let key_vec = vec![
            (Key::Directional(Direction::Up), IVec2 { x: 1, y: 0 }),
            (Key::A, IVec2 { x: 2, y: 0 }),
            (Key::Directional(Direction::Left), IVec2 { x: 0, y: 1 }),
            (Key::Directional(Direction::Down), IVec2 { x: 1, y: 1 }),
            (Key::Directional(Direction::Right), IVec2 { x: 2, y: 1 }),
        ];
        key_vec.iter().for_each(|(key, pos)| {
            _ = keys.insert(*key, *pos);
            _ = positions.insert(*pos, *key);
        });

        Keypad { keys, positions }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cursors: Vec<IVec2>,
    output: String,
}

fn press_button(
    state: &mut State,
    keypads: &[Keypad],
    button: Key,
    index: usize,
) -> Result<(), ()> {
    match button {
        Key::Numeric(c) => state.output.push(c.to_string().chars().next().unwrap()),
        Key::Directional(direction) => state.cursors[index] += direction.to_ivec(),
        Key::A => {
            if index == keypads.len() {
                state.output.push('A');
                return Ok(());
            }
            let cursor = state.cursors[index];
            let button = keypads[index].positions.get(&cursor);
            if button.is_none() {
                return Err(());
            }
            let button = button.unwrap();
            return press_button(state, keypads, *button, index + 1);
        }
    }
    Ok(())
}

fn succ(state: &State, keypads: &Vec<Keypad>, target: &str) -> Vec<(State, u64)> {
    let possible_buttons = vec![
        // Human pressable buttons
        Key::Directional(Direction::Up),
        Key::Directional(Direction::Down),
        Key::Directional(Direction::Left),
        Key::Directional(Direction::Right),
        Key::A,
    ];
    let mut next_states = vec![];
    if !target.starts_with(state.output.as_str()) {
        return next_states;
    }

    for key in possible_buttons {
        match key {
            Key::Numeric(_) => {}
            Key::Directional(direction) => {
                let newpos = state.cursors[0] + direction.to_ivec();
                if keypads[0].positions.contains_key(&newpos) {
                    let mut next_state = state.clone();
                    next_state.cursors[0] = newpos;
                    next_states.push((next_state, 1));
                }
            }
            Key::A => {
                let mut new_state = state.clone();
                let r = press_button(&mut new_state, keypads, Key::A, 0);
                if r.is_ok() {
                    next_states.push((new_state, 1));
                }
            }
        };
    }

    next_states
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let keypads = vec![
        Keypad::new_directional(),
        Keypad::new_directional(),
        Keypad::new_numeric(),
    ];
    let cursors = vec![
        IVec2 { x: 2, y: 0 },
        IVec2 { x: 2, y: 0 },
        IVec2 { x: 2, y: 2 },
    ];
    let initial_state = State {
        cursors,
        output: "".to_string(),
    };
    let sequences = parse(input);

    let seq = sequences[0];

    dijkstra(
        &initial_state,
        |x| succ(x, &keypads, seq),
        |x| x.output == seq,
    );
    dbg!(sequences);
    todo!()
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
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
        assert_eq!(result, "126384"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, ""); // Replace with the actual expected result
    }
}
