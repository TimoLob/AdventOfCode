use core::panic;
use std::collections::HashMap;

use glam::IVec2;

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
    Gap,
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

struct Keypad {
    movements: HashMap<(Key, Key), IVec2>,
    keys: HashMap<Key, IVec2>,
    current: IVec2,
}
impl Keypad {
    fn new_numeric() -> Self {
        let mut movements = HashMap::new();

        let mut keys = HashMap::new();
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
            (Key::Gap, IVec2 { x: 0, y: 2 }),
            (Key::Numeric(0), IVec2 { x: 1, y: 2 }),
            (Key::A, IVec2 { x: 2, y: 2 }),
        ];
        key_vec
            .iter()
            .for_each(|(key, pos)| _ = keys.insert(*key, *pos));

        for (&skey, start_pos) in keys.iter() {
            for (&tkey, target_pos) in keys.iter() {
                let direction = target_pos - start_pos;

                movements.insert((skey, tkey), direction);
            }
        }
        dbg!(&keys, &movements);
        let current = IVec2 { x: 2, y: 2 };
        Keypad {
            keys,
            movements,
            current,
        }
    }

    fn new_directional() -> Self {
        let mut movements = HashMap::new();

        let mut keys = HashMap::new();
        let key_vec = vec![
            (Key::Gap, IVec2 { x: 0, y: 0 }),
            (Key::Directional(Direction::Up), IVec2 { x: 1, y: 0 }),
            (Key::A, IVec2 { x: 2, y: 0 }),
            (Key::Directional(Direction::Left), IVec2 { x: 0, y: 1 }),
            (Key::Directional(Direction::Down), IVec2 { x: 1, y: 1 }),
            (Key::Directional(Direction::Right), IVec2 { x: 2, y: 1 }),
        ];
        key_vec
            .iter()
            .for_each(|(key, pos)| _ = keys.insert(*key, *pos));

        for (&skey, start_pos) in keys.iter() {
            for (&tkey, target_pos) in keys.iter() {
                let direction = target_pos - start_pos;

                movements.insert((skey, tkey), direction);
            }
        }
        dbg!(&keys, &movements);
        let current = IVec2 { x: 2, y: 0 };
        Keypad {
            keys,
            movements,
            current,
        }
    }

    fn execute(&mut self, key: Key) -> Result<(), ()> {
        match key {
            Key::Directional(direction) => {
                let dir = direction.to_ivec();
                let new_pos = self.current + dir;
                if let Some(key) = self
                    .keys
                    .iter()
                    .filter(|(_k, pos)| **pos == new_pos)
                    .map(|(k, _p)| k)
                    .next()
                {
                    if *key == Key::Gap {
                        return Err(());
                    }
                    self.current = new_pos;
                    return Ok(());
                } else {
                    return Err(());
                }
            }
            _ => return Err(()),
        };
    }
}

fn parse(input: &str) -> Vec<Vec<Key>> {
    input
        .lines()
        .map(|line| {
            let mut button_presses = vec![];
            for c in line.chars() {
                match c {
                    'A' => button_presses.push(Key::A),
                    c if ('0'..='9').contains(&c) => {
                        button_presses.push(Key::Numeric(c.to_digit(10).unwrap() as usize))
                    }

                    c => panic!("unexpected char {}", c),
                };
            }
            button_presses
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let keypads = vec![
        Keypad::new_directional(),
        Keypad::new_directional(),
        Keypad::new_numeric(),
    ];
    let sequences = parse(input);
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
