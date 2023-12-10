use nom::{self, multi::{separated_list1, fold_many1}, character::complete::{newline, space1, space0}, sequence::{tuple, preceded, terminated}, character::complete, IResult, bytes::complete::tag};
use std::iter::zip;
struct Race {
    time:u128,
    dist:u128
}

fn parse(input:&str) -> IResult<&str,Vec<Race>> {
    let mut lines = input.lines();
    let times_str = lines.next().expect("First line");
    let distance_str = lines.next().expect("Second line");
    let (_,times) = preceded(tuple((tag("Time:"),space1)),fold_many1(terminated(complete::u128, space0), 
    Vec::new, |mut acc:Vec<u128>, item| {
        acc.push(item);
        acc
    }))(times_str)?;
    let (_,distances) = preceded(tuple((tag("Distance:"),space1)),fold_many1(terminated(complete::u128, space0), 
    Vec::new, |mut acc:Vec<u128>, item| {
        acc.push(item);
        acc
    }))(distance_str)?;
    println!("Times: {:?}",times);
    println!("Dsit: {:?}",distances);

    let mut races: Vec<Race> = vec![];
    for (time,dist) in zip(times,distances) {
        races.push(Race { time: time, dist: dist });
    }
    Ok((input,races))
}


fn evaluate_race(race : &Race) -> u128 {
    let mut lower_bound = 0;
    let mut upper_bound = 0;
    for x in 1..race.time {
        let charge_time = x;
        let remaining_time = race.time-charge_time;
        let speed = charge_time;
        let distance = remaining_time*speed;
        if distance>race.dist {
            lower_bound = x;
            break;
        }
    }

    for x in (1..race.time).rev() {
        let charge_time = x;
        let remaining_time = race.time-charge_time;
        let speed = charge_time;
        let distance = remaining_time*speed;
        if distance>race.dist {
            upper_bound = x+1;
            break;
        }
    }
    return upper_bound-lower_bound;
}

fn solve(input: &str) -> u128 {
    let (_,races)=parse(input).expect("Should parse");
    let ways_to_win:Vec<u128> = races.iter().map(|race| evaluate_race(race)).collect();
    let result = ways_to_win.iter().fold(1, |acc,element| acc*element);
    println!("{:?}",ways_to_win);
    return result;
}

fn main() {
    println!("Hello, world!");
    let input = include_str!("../../input2.txt");
    let result = solve(input);
    println!("Result {}",result);

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let example = include_str!("../../example2.txt");
        assert_eq!(solve(example), 71503);
    }
}
