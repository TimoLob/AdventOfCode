use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let mut lines = input.lines();

    let start = lines.next().unwrap().chars().position(|c| c=='S').unwrap();
    let mut splitter_lines :Vec<Vec<usize>> =  Vec::new();
    lines.for_each(|line|{
        let splits = 
        line.chars().enumerate().filter_map(|(i,c)| {if c=='^' {Some(i)} else {None}}).collect();

        splitter_lines.push(splits);
        
    });
    //println!("{:?}",splitter_lines);
    let mut lasers : HashSet<usize> = HashSet::new();
    let mut num_splits =0;
    lasers.insert(start);
    splitter_lines.iter().for_each(|splitters| {
        if splitters.is_empty() {
            return;
        }
        let mut new_lasers:HashSet<usize> = HashSet::new();
        for &laser in lasers.iter() {
            match splitters.binary_search(&laser) {
                Ok(_) => {
                    new_lasers.insert(laser-1);
                    new_lasers.insert(laser+1);
                    num_splits+=1;
                },
                Err(_) => {new_lasers.insert(laser);},
            }

        }
        lasers = new_lasers;
        //println!("{:?}",lasers);

    });
    num_splits.to_string()
}

#[derive(Debug,Clone, Copy)]
struct Laser {
    position:usize,
    universes:usize
}


pub fn part2(input: &str) -> String {
    let input = input.trim();
    let input = input.trim();
    let mut lines = input.lines();

    let start = lines.next().unwrap().chars().position(|c| c=='S').unwrap();
    let mut splitter_lines :Vec<Vec<usize>> =  Vec::new();
    lines.for_each(|line|{
        let splits = 
        line.chars().enumerate().filter_map(|(i,c)| {if c=='^' {Some(i)} else {None}}).collect();

        splitter_lines.push(splits);
        
    });
    //println!("{:?}",splitter_lines);
    let mut lasers : Vec<Laser> = Vec::new();
    lasers.push(Laser { position: start, universes: 1 });
    splitter_lines.iter().for_each(|splitters| {
        if splitters.is_empty() {
            return;
        }
        let mut new_lasers:Vec<Laser> = Vec::new();
        for &laser in lasers.iter() {
            match splitters.binary_search(&laser.position) {
                Ok(_) => {
                    let pos = new_lasers.iter().position(|l| l.position == laser.position-1);
                    match pos {
                        Some(idx) => new_lasers[idx].universes+=laser.universes,
                        None => new_lasers.push(Laser { position: laser.position-1, universes: laser.universes }),
                    };
                    let pos = new_lasers.iter().position(|l| l.position == laser.position+1);
                    match pos {
                        Some(idx) => new_lasers[idx].universes+=laser.universes,
                        None => new_lasers.push(Laser { position: laser.position+1, universes: laser.universes }),
                    };
                },
                Err(_) => {
                    let pos = new_lasers.iter().position(|l| l.position == laser.position);
                    match pos {
                        Some(idx) => new_lasers[idx].universes+=laser.universes,
                        None => new_lasers.push(Laser { position: laser.position, universes: laser.universes }),
                    };
                },
            }

        }
        lasers = new_lasers;

    });
    lasers.iter().map(|l| l.universes).sum::<usize>().to_string()
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "21");
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "40"); // Replace with the actual expected result
    }
}
