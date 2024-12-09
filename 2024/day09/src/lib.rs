use std::{cmp::min, usize};
#[derive(Debug, Copy, Clone)]
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

fn is_free(file: &File) -> bool {
    return file.id == usize::MAX;
}

pub fn part2(input: &str) -> String {
    /*
     * Parse input into Vec of files
     * id = usize::MAX indicates empty space
     * */
    let input = input.trim();
    let mut files: Vec<File> = Vec::new();
    input.chars().enumerate().for_each(|(idx, c)| {
        let size = c.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            let f = File { size, id: idx / 2 };
            files.push(f);
        } else {
            let f = File {
                size,
                id: usize::MAX,
            };
            files.push(f);
        }
    });

    /*
     * Init : Highest file = last file
     * Loop until highest file.id = 0
     *  Loop through files from start to end to find whitespace that can fit highest file
     *      if exact fit, swap ids, break
     *      if whitespace bigger than highest file, reduce space by size of highest file
     *          then set highest file to be whitespace
     *          then insert highest file before whitespace
     *          break
     *  Loop backwards through the list
     *   If 2 Empty spaces in a row, add sizes into lower index one and set the higher index one to 0
     *  Loop through indices marked for deletion and delete
     *  Loop to find next highest file id
     * */
    let mut highest_file_idx = files.len() - 1;
    let mut highest_file = files[highest_file_idx];

    while highest_file.id > 0 {
        for i in 0..highest_file_idx - 1 {
            let file = files[i]; // Empty Space
            if is_free(&file) && file.size == highest_file.size {
                files[i].id = highest_file.id;
                files[highest_file_idx].id = usize::MAX;
                break;
            }
            if is_free(&file) && file.size > highest_file.size {
                files[i].size -= highest_file.size;
                files[highest_file_idx].id = usize::MAX;
                files.insert(i, highest_file);
                break;
            }
        }

        //let mut indices_to_remove: Vec<usize> = vec![];
        for i in (1..files.len()).rev() {
            let f1 = files[i];
            let f2 = files[i - 1];

            if is_free(&f1) && is_free(&f2) {
                files[i - 1].size += f1.size;
                //indices_to_remove.push(i);
                files[i].size = 0;
            }
        }

        // Find next highest file  & index
        for i in 0..files.len() {
            if files[i].id == highest_file.id - 1 {
                highest_file_idx = i;
                highest_file = files[i];
                break;
            }
        }
    }
    let mut index = 0;
    let mut total = 0;
    for i in 0..files.len() {
        let file = files[i];
        let id = if is_free(&file) { 0 } else { file.id }; // if empty, set id to 0
        for _ in 0..file.size {
            total += index * id;
            index += 1;
        }
    }
    total.to_string()
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
        assert_eq!(result, "2858"); // Replace with the actual expected result
    }
}
