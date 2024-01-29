use core::panic;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn score_win(me: Hand, opp: Hand) -> u32 {
    if me == opp {
        return 3;
    }
    match me {
        Hand::Rock => {
            if opp == Hand::Paper {
                0
            } else {
                6
            }
        }
        Hand::Paper => {
            if opp == Hand::Scissors {
                0
            } else {
                6
            }
        }
        Hand::Scissors => {
            if opp == Hand::Rock {
                0
            } else {
                6
            }
        }
    }
}

fn solve1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (opp, me) = line.split_once(' ').unwrap();
            let opp = opp.chars().next().unwrap();
            let me = me.chars().next().unwrap();
            let opp = match opp {
                'A' => Hand::Rock,
                'B' => Hand::Paper,
                'C' => Hand::Scissors,
                c => panic!("Inavalid char {}", c),
            };
            let me = match me {
                'X' => Hand::Rock,
                'Y' => Hand::Paper,
                'Z' => Hand::Scissors,
                c => panic!("Inavalid char {}", c),
            };
            score_win(me, opp)
                + match me {
                    Hand::Rock => 1,
                    Hand::Paper => 2,
                    Hand::Scissors => 3,
                }
        })
        .sum::<u32>()
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1 : {}", solve1(input));
}
