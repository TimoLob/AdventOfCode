use std::collections::HashSet;

use glam::{I64Vec3, IVec3};

type JunctionBox = I64Vec3;
type Circuit = HashSet<JunctionBox>;

#[derive(Debug)]
struct Distance {
    i1 : usize,
    i2 : usize,
    dist: i64
}


impl Distance {
    fn from(a: JunctionBox, ai:usize, b : JunctionBox, bi: usize) -> Self {
        let dist = a.distance_squared(b);
        Distance { i1: ai, i2: bi, dist : dist }
    }
}

pub fn top3<I>(iter: I) -> [usize; 3]
where
    I: IntoIterator<Item = usize>,
{
    let mut top = [0usize; 3]; // top[0] = largest

    for x in iter {
        if x > top[0] {
            top[2] = top[1];
            top[1] = top[0];
            top[0] = x;
        } else if x > top[1] {
            top[2] = top[1];
            top[1] = x;
        } else if x > top[2] {
            top[2] = x;
        }
    }

    top
}

fn connect_and_merge_circuits(junction_boxes: &Vec<JunctionBox>, n : usize) -> usize {
    
    let mut circuits = junction_boxes.iter().map(|junctionbox| {
        let mut c = Circuit::new();
        c.insert(*junctionbox);
        c

    }).collect::<Vec<Circuit>>();

    let mut distances : Vec<Distance> = Vec::with_capacity(junction_boxes.len()*junction_boxes.len());
    for i in 0..junction_boxes.len() {
        for j in (i+1)..junction_boxes.len() {
            distances.push(Distance::from(junction_boxes[i], i, junction_boxes[j], j));
        }
    }
    distances.sort_by_key(|a| a.dist);
    //distances.iter().for_each(|d| {
    //    println!("{:?}->{:?} = {}",junction_boxes[d.i1],junction_boxes[d.i2], d.dist);
    //});


    for i in 0..n {
        let d = &distances[i];
        let a = junction_boxes[d.i1];
        let b = junction_boxes[d.i2];
        let set_a = circuits.iter_mut().position(|s| s.contains(&a)).expect("All junction boxes should be part of a circuit.");
        let set_b = circuits.iter().position(|s| s.contains(&b)).expect("All junction boxes should be part of a circuit.");
        if set_a == set_b {
            continue;
        }
        let (keep, remove) = if set_a < set_b {
            (set_a, set_b)
        } else {
            (set_b, set_a)
        };
        {
            let (left, right) = circuits.split_at_mut(remove);
            let  set_keep = &mut left[keep];
            let set_remove = &mut right[0];
            set_keep.extend(set_remove.drain());
        }
        circuits.remove(remove);
    }
    top3(circuits.iter().map(|set| set.len())).iter().fold(1,|acc,x| acc*x)
}


pub fn part1(input: &str) -> String {
    let input = input.trim();
    let junction_boxes = input.lines().map(|line| {
        let splits = line.splitn(3, ',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        JunctionBox::from_slice(splits.as_slice())
    }).collect::<Vec<JunctionBox>>();

    
    connect_and_merge_circuits(&junction_boxes, 1000).to_string()
    
}
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let junction_boxes = input.lines().map(|line| {
        let splits = line.splitn(3, ',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        JunctionBox::from_slice(splits.as_slice())
    }).collect::<Vec<JunctionBox>>();


    let mut circuits = junction_boxes.iter().map(|junctionbox| {
        let mut c = Circuit::new();
        c.insert(*junctionbox);
        c

    }).collect::<Vec<Circuit>>();

    let mut distances : Vec<Distance> = Vec::with_capacity(junction_boxes.len()*junction_boxes.len());
    for i in 0..junction_boxes.len() {
        for j in (i+1)..junction_boxes.len() {
            distances.push(Distance::from(junction_boxes[i], i, junction_boxes[j], j));
        }
    }
    distances.sort_by_key(|a| a.dist);

    for i in 0..distances.len() {
        let d = &distances[i];
        let a = junction_boxes[d.i1];
        let b = junction_boxes[d.i2];
        let set_a = circuits.iter_mut().position(|s| s.contains(&a)).expect("All junction boxes should be part of a circuit.");
        let set_b = circuits.iter().position(|s| s.contains(&b)).expect("All junction boxes should be part of a circuit.");
        if set_a == set_b {
            continue;
        }
        let (keep, remove) = if set_a < set_b {
            (set_a, set_b)
        } else {
            (set_b, set_a)
        };
        {
            let (left, right) = circuits.split_at_mut(remove);
            let  set_keep = &mut left[keep];
            let set_remove = &mut right[0];
            set_keep.extend(set_remove.drain());
        }
        circuits.remove(remove);
        if circuits.len() == 1 {
            return (a.x * b.x).to_string();
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let input = input.trim();
        let junction_boxes = input.lines().map(|line| {
            let splits = line.splitn(3, ',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            JunctionBox::from_slice(splits.as_slice())
        }).collect::<Vec<JunctionBox>>();

        
        let result = connect_and_merge_circuits(&junction_boxes, 10).to_string();
        assert_eq!(result, "40"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "25272"); // Replace with the actual expected result
    }
}
