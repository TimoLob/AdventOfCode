use rayon::prelude::*;

fn prng(mut secret: i64) -> i64 {
    let r = secret * 64;
    secret = secret ^ r;
    secret = secret % 16777216;

    let r = secret / 32;
    secret = secret ^ r;
    secret = secret % 16777216;

    let r = secret * 2048;
    secret = secret ^ r;
    secret = secret % 16777216;
    secret
}

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &str) -> String {
    let input = input.trim();
    let mut secret_numbers = parse(input);
    for _ in 0..2000 {
        secret_numbers = secret_numbers.iter().map(|x| prng(*x)).collect();
    }
    let total: i64 = secret_numbers.iter().sum();
    total.to_string()
}

fn gen_all_possible_sequences() -> Vec<Vec<i64>> {
    let mut seq = Vec::new();
    for i in -9..=9 {
        for j in -9..=9 {
            for k in -9..=9 {
                for l in -9..=9 {
                    seq.push(vec![i, j, k, l]);
                }
            }
        }
    }
    return seq;
}

fn calculate_profit(secret_sequences: &Vec<Vec<(i64, i64)>>, target_sequence: &Vec<i64>) -> i64 {
    let mut profit = 0;

    for sequence in secret_sequences {
        for window in sequence.windows(4) {
            let mut fit = true;
            for i in 0..4 {
                if window[i].1 != target_sequence[i] {
                    fit = false;
                }
            }
            if fit {
                profit += window[3].0;
                break;
            }
        }
    }

    profit
}

pub fn part2(input: &str) -> String {
    let input = input.trim();

    let secret_numbers = parse(input);
    let mut secret_sequences: Vec<Vec<(i64, i64)>> = Vec::new();
    secret_numbers.iter().for_each(|secret| {
        let mut seq = Vec::new();
        let mut price = secret % 10;
        let mut current_secret = *secret;
        for _ in 0..2000 {
            let new_secret = prng(current_secret);
            let new_price = new_secret % 10;
            let diff = new_price - price;
            seq.push((new_price, diff));
            price = new_price;
            current_secret = new_secret;
        }
        secret_sequences.push(seq);
    });
    let targets = gen_all_possible_sequences();
    let best = targets
        .par_iter()
        .map(|target| calculate_profit(&secret_sequences, target))
        .max()
        .unwrap();
    best.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "37327623"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example2.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "23"); // Replace with the actual expected result
    }

    #[test]
    fn secret_numbers() {
        let mut secret = 123;
        let next_secrets = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for i in 0..next_secrets.len() {
            secret = prng(secret);
            dbg!(i, secret);
            assert_eq!(secret, next_secrets[i]);
        }
    }

    #[test]
    fn test_profit() {
        let input = fs::read_to_string("example2.txt").expect("Failed to read example.txt");
        let secret_numbers = parse(&input);
        let mut secret_sequences: Vec<Vec<(i64, i64)>> = Vec::new();
        secret_numbers.iter().for_each(|secret| {
            let mut seq = Vec::new();
            let mut price = secret % 10;
            let mut current_secret = *secret;
            for _ in 0..2000 {
                let new_secret = prng(current_secret);
                let new_price = new_secret % 10;
                let diff = new_price - price;
                seq.push((new_price, diff));
                price = new_price;
                current_secret = new_secret;
            }
            secret_sequences.push(seq);
        });
        let target = vec![-2, 1, -1, 3];
        let profit = calculate_profit(&secret_sequences, &target);
        assert_eq!(profit, 23);
    }
}
