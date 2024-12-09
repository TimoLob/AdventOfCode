#[derive(Debug)]
struct File {
    size: usize,
    id: usize,
    index: usize,
}

#[derive(Debug)]
struct FreeSpace {
    size: usize,
    index: usize,
}

enum ParseState {
    ReadFile,
    ReadFree,
}

fn parse(input: &str) -> (Vec<FreeSpace>, Vec<File>, usize) {
    let input = input.trim();
    let mut parsestate = ParseState::ReadFile;
    let mut fileid = 0;
    let mut total_free_space = 0;
    let mut free_spaces: Vec<FreeSpace> = Vec::new();
    let mut files: Vec<File> = Vec::new();
    let mut index = 0;
    input.chars().for_each(|c| match parsestate {
        ParseState::ReadFile => {
            parsestate = ParseState::ReadFree;
            let size = c.to_digit(10).unwrap() as usize;
            let file = File {
                size,
                id: fileid,
                index,
            };
            fileid += 1;
            index += 1;
            files.push(file);
        }
        ParseState::ReadFree => {
            parsestate = ParseState::ReadFile;
            let space = c.to_digit(10).unwrap() as usize;
            total_free_space += space;
            let free = FreeSpace { size: space, index };
            index += 1;
            free_spaces.push(free);
        }
    });
    free_spaces.push(FreeSpace { size: 0, index });
    (free_spaces, files, total_free_space)
}

pub fn part1(input: &str) -> String {
    let (free_spaces, files, total_free_space) = parse(input);

    let mut string_repr = "".to_string();
    let mut index = 0;
    let mut i = 0;
    let mut j = 0;
    let mut current_end_space = 0;
    while index < free_spaces.last().unwrap().index {
        let file = &files[i];
        let free = &free_spaces[j];
        if file.index == index {
            for _ in 0..file.size {
                string_repr.push(file.id.to_string().chars().next().unwrap());
            }
            i += 1;
        }
        if free.index == index {
            for _ in 0..free.size {
                string_repr.push('.');
            }
            j += 1;
        }
        index += 1;
    }

    dbg!(&string_repr);
    let mut string_repr: Vec<char> = string_repr.chars().collect();
    println!();
    while current_end_space != total_free_space {
        let first_dot = string_repr.iter().position(|&c| c == '.').unwrap();
        let last_num =
            string_repr.len() - string_repr.iter().rev().position(|&c| c != '.').unwrap() - 1;
        string_repr[first_dot] = string_repr[last_num];
        string_repr[last_num] = '.';
        current_end_space += 1;
    }

    let mut total = 0;
    for (idx, c) in string_repr.iter().enumerate() {
        if !c.is_numeric() {
            continue;
        }
        let num = c.to_digit(10).unwrap();
        total += idx * num as usize;
    }

    dbg!(&string_repr);
    total.to_string()
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
        assert_eq!(result, "1928"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, ""); // Replace with the actual expected result
    }
}
