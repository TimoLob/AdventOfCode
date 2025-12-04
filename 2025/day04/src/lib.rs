use std::collections::{HashMap, HashSet};

use glam::IVec2;

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let mut map : HashSet<IVec2> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {map.insert(IVec2 { x: x as i32, y: y as i32 });}
                _ => {}
            };
        }
    }

    let offsets = vec![
        IVec2::new(-1, 0),
        IVec2::new(1,0),
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(-1, -1),
        IVec2::new(1, -1),
        IVec2::new(-1, 1),
        IVec2::new(1, 1),
    ];
    let mut result = 0;
    for pos in map.iter() {
        let mut neighbors = 0;
        for offset in offsets.iter() {
            if map.contains(&(pos + offset)) {
                neighbors+=1;
            }
        }
        if neighbors < 4 {
            result += 1;
        }
    }
    result.to_string()
}


pub fn part2(input: &str) -> String {
    let input = input.trim();
    let mut set : HashSet<IVec2> = HashSet::new();
    let mut map : HashMap<IVec2,i32> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    set.insert(IVec2 { x: x as i32, y: y as i32 });
                    map.insert(IVec2 { x: x as i32, y: y as i32 },0);

                }
                _ => {}
            };
        }
    }
    let offsets = vec![
        IVec2::new(-1, 0),
        IVec2::new(1,0),
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(-1, -1),
        IVec2::new(1, -1),
        IVec2::new(-1, 1),
        IVec2::new(1, 1),
    ];
    for pos in set.iter() {
        let mut neighbors = 0;
        for offset in offsets.iter() {
            if set.contains(&(pos + offset)) {
                neighbors+=1;
            }
        }
        let entry = map.get_mut(pos).expect("Should exist");
        *entry = neighbors;
    }
    let mut to_remove : HashSet<IVec2> = HashSet::new();
    for (k,v) in map.iter() {
        if *v < 4 {
            to_remove.insert(k.clone());
        }
    }
    let mut reachable = 0;
    while to_remove.len() > 0 {
        let roll = to_remove.iter().next().unwrap().clone();
        reachable+=1;
        to_remove.remove(&roll);
        map.remove(&roll);
        for offset in offsets.iter() {
            let entry = map.get_mut(&(roll+offset));
            match entry {
                Some(n) => {
                    *n-=1;
                    if *n < 4 {
                        to_remove.insert(roll+offset);
                    }
                }
                None => {}
            }
        }
        
    }
    reachable.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "13"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "43"); // Replace with the actual expected result
    }
}
