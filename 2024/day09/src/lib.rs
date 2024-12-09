use std::cmp::min;
#[derive(Debug)]
struct File {
    size: usize,
    id: usize,
}

pub fn part1(input: &str) -> String {
    let input = input.trim();

    let mut files: Vec<File> = Vec::new();
    input.chars().enumerate().for_each(|(idx, c)| {
        if idx % 2 == 0 {
            let f = File {
                size: c.to_digit(10).unwrap() as usize,
                id: idx / 2,
            };
            files.push(f);
        }
    });
    let mut whitespace: Vec<usize> = Vec::new();
    input.chars().enumerate().for_each(|(idx, c)| {
        if idx % 2 == 1 {
            whitespace.push(c.to_digit(10).unwrap() as usize);
        }
    });
    let mut last_file = files.len() - 1;
    let mut disk: Vec<usize> = Vec::new();

    let mut i = 0; // Index into files
    let mut j = 0; // Index into whitespace

    let mut index = 0;
    while i <= last_file {
        if index % 2 == 0 {
            let f = &files[i];
            for _ in 0..f.size {
                disk.push(f.id);
            }
            i += 1;
            index += 1;
        } else if index % 2 == 1 {
            let empty = whitespace[j];
            let f = files.get_mut(last_file).unwrap();
            let size_to_copy = min(empty, f.size);
            let remaining_size = f.size - size_to_copy;
            if remaining_size == 0 {
                last_file -= 1;
            } else {
                f.size = remaining_size;
            }
            if size_to_copy == empty {
                index += 1;
                j += 1;
            } else {
                whitespace[j] = empty - size_to_copy;
            }
            for _ in 0..size_to_copy {
                disk.push(f.id);
            }
        }
    }
    let checksum: usize = disk.iter().enumerate().map(|(idx, f)| idx * f).sum();
    checksum.to_string()
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
