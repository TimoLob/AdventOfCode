
pub fn part1(input: &str) -> String {
    const MAX_LINE_LENGTH:usize = 150; // Magic number. Should be larger or equal to the line length of input
    let input = input.trim();
    let mut lines = input.lines();

    let start = lines.next().unwrap().chars().position(|c| c=='S').unwrap();
    let mut splitter_lines :Vec<Vec<usize>> =  Vec::new();
    lines.for_each(|line|{
        let splits = 
        line.chars().enumerate().filter_map(|(i,c)| {if c=='^' {Some(i)} else {None}}).collect();

        splitter_lines.push(splits);
        
    });
    
    let mut lasers : Vec<bool> = vec![false; MAX_LINE_LENGTH];
    lasers[start] = true;
    let mut num_splits = 0;
    splitter_lines.iter().for_each(|splitters| {
        if splitters.is_empty() {
            return;
        }
        let mut new_lasers: Vec<bool> = vec![false; MAX_LINE_LENGTH];

        for (pos,&_) in lasers.iter().enumerate().filter(|(_,laser)| **laser) {

            // Splitters are sorted
            match splitters.binary_search(&pos) {
                Ok(_) => {
                    new_lasers[pos-1] = true;
                    new_lasers[pos+1] = true;
                    num_splits += 1;
                },
                Err(_) => {
                    new_lasers[pos] = true;
                },
            }

        }
        lasers = new_lasers;

    });
    num_splits.to_string()
}



pub fn part2(input: &str) -> String {
    const MAX_LINE_LENGTH:usize = 150; // Magic number. Should be larger or equal to the line length of input
    let input = input.trim();
    let mut lines = input.lines();

    let start = lines.next().unwrap().chars().position(|c| c=='S').unwrap();
    let mut splitter_lines :Vec<Vec<usize>> =  Vec::new();
    lines.for_each(|line|{
        let splits = 
        line.chars().enumerate().filter_map(|(i,c)| {if c=='^' {Some(i)} else {None}}).collect();

        splitter_lines.push(splits);
        
    });
    
    let mut lasers : Vec<usize> = vec![0; MAX_LINE_LENGTH];
    lasers[start] =1;

    splitter_lines.iter().for_each(|splitters| {
        if splitters.is_empty() {
            return;
        }
        let mut new_lasers: Vec<usize> = vec![0; MAX_LINE_LENGTH];

        for (pos,&universes) in lasers.iter().enumerate().filter(|(_,universes)| **universes!=0) {

            // Splitters are sorted
            match splitters.binary_search(&pos) {
                Ok(_) => {
                    new_lasers[pos-1] += universes;
                    new_lasers[pos+1] += universes;
                },
                Err(_) => {
                    new_lasers[pos] += universes;
                },
            }

        }
        lasers = new_lasers;

    });
    lasers.iter().sum::<usize>().to_string()
    
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
