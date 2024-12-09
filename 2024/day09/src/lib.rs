use std::cmp::min;

pub fn part1(input: &str) -> String {
    let input = input.trim();

    let mut files: Vec<usize> = Vec::new();
    input.chars().enumerate().for_each(|(idx, c)| {
        if idx % 2 == 0 {
            for _ in 0..c.to_digit(10).unwrap() {
                files.push(idx / 2);
            }
        }
    });
    let mut last_file = files.len() - 1;
    let mut disk: Vec<usize> = Vec::new();
    let mut files_processed = 0;
    input.chars().enumerate().for_each(|(idx, c)| {
        let size = c.to_digit(10).unwrap();

        if idx % 2 == 0 && files_processed < last_file {
            for _ in 0..min(size as usize, last_file - files_processed) {
                disk.push(idx / 2);
                files_processed += 1;
            }
        } else if idx % 2 == 1 {
            for _ in 0..size {
                if last_file > idx {
                    disk.push(files[last_file]);
                    last_file -= 1;
                }
            }
        }
    });
    dbg!(&disk);

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
