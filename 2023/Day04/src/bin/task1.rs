use nom::{self, sequence::preceded, character::complete::{digit1, line_ending}, bytes::complete::tag, IResult, multi::{separated_list1, many1} };
#[derive(Debug)]
struct Game {
    winning_numbers : Vec<u32>,
    given_numbers : Vec<u32>,
    id : u32,
}

impl Game {
    fn value(&self) -> u32 {
        let mut score = 0;
        for num in &self.given_numbers {
            if self.winning_numbers.contains(&num) {
                if score == 0 {
                    score = 1;
                }
                else {
                    score *=2;
                }
            }
        }
        return score;
    }
    
}
// 10  5 11 65 27 43 44 29 24 69
fn parse_cards(input : &str) -> IResult<&str, Vec<u32>> {

    let (input, v) = separated_list1(many1(tag(" ")), digit1)(input)?;
    Ok((input,v.iter().map(|x| x.parse::<u32>().unwrap()).collect()))

}


// Card   1: 10  5 11 65 27 43 44 29 24 69 | 65 66 18 14 17 97 95 34 38 23 10 25 22 15 87  9 28 43  4 71 89 20 72  5  6
fn parse_game(input:&str) -> IResult<&str, Game> {
   let (input,id) = preceded(preceded(tag("Card"), many1(tag(" "))), digit1)(input)?;
   let (input,_) = preceded(tag(":"), many1(tag(" ")))(input)?;

   let (input, v) = separated_list1(preceded(preceded(many1(tag(" ")), tag("|")),many1(tag(" "))), parse_cards) (input)?;
   let game = Game{winning_numbers:v[0].clone(),given_numbers:v[1].clone(),id:id.parse::<u32>().unwrap()};

   Ok((input,game))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) =
        separated_list1(line_ending, parse_game)(input)?;
    Ok((input,games))
}
fn solve(input:&str) -> u32 {
    //let result = input.lines().map(|x| parse_line(x).value()).sum::<u32>();
    let (_,games) = parse_games(input).expect("Should be parsable");
    println!("{:?}",games);
    games.iter().map(|game| game.value()).sum::<u32>()
}

fn main() {
    let input = include_str!("..//../input.txt");
    
   println!("Result:{}",solve(input));
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example() {
        let example_input = include_str!("../../test.txt");
        assert_eq!(solve(example_input),13);
    }
}
