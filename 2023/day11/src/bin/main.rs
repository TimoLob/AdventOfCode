fn dist(g1: (usize,usize),g2: (usize,usize)) -> usize {
    return g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
}

fn solve(input: &str) -> usize {
    let mut x_offsets : Vec<usize> = vec![];
    let mut y_offsets : Vec<usize> = vec![];
    let input :Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = input[0].len();
    let height = input.len();
    // Iterate in x Richting
    for x in 0..width {
        let mut empty = true;
        for y in 0..height {
            if input[y][x] == '#' {
                empty = false;
                break;
            }
        }
        // let x_offset = if x == 0 {0} else {x_offsets[x-1]+1+(if empty {1} else {0} )}; // Part 1
        let x_offset = if x == 0 {0} else {x_offsets[x-1]+1+(if empty {1000000-1} else {0} )};
        x_offsets.push(x_offset)
    }
    // Iterate in y direction
    for y in 0..height {
        let mut empty = true;
        for x in 0..width {
            if input[y][x] == '#' {
                empty = false;
                break;
            }
        }
        // let y_offset = if y==0 {0} else { y_offsets[y-1]+1+(if empty {1} else {0})}; // Part 1
        let y_offset = if y==0 {0} else { y_offsets[y-1]+1+(if empty {1000000-1} else {0})};
        y_offsets.push(y_offset);
    }

    let mut galaxies : Vec<(usize,usize)> = vec![];

    for x in 0..width {
        for y in 0..height {
            if input[y][x] == '#' {
                galaxies.push((x_offsets[x],y_offsets[y]));
            }
        }
    }

    //println!("{:?}",galaxies);
    let mut total :usize = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            total += dist(galaxies[i], galaxies[j]);
        }
    }
    return total;
}


fn main() {
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result: {}",result);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let example = include_str!("../../example.txt");
        assert_eq!(solve(example), 374);
    }
    
}
