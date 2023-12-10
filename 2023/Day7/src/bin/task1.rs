use std::{string::ParseError, collections::{HashSet, HashMap}, cmp::Ordering};

use nom::{multi::separated_list1, character::complete::{ space1, line_ending},sequence::separated_pair, bytes::complete::take, IResult, ToUsize};



#[derive(Debug)]
struct Hand {
    cards : [u32;5],
    bid : u64
}

fn five_of_a_kind(map : &HashMap<u32,u32>)-> bool {
    map.len() == 1
}

fn four_of_a_kind(map : &HashMap<u32,u32>) -> bool {
    for (_,value) in map.iter() {
        if *value == 4 {
            return true;
        }
    }
    return false;
}

fn full_house(map : &HashMap<u32,u32>) -> bool {
    if map.len() == 2 {
        let values : Vec<u32> = map.values().map(|x| *x).collect();
        if values.contains(&2) && values.contains(&3) {
            return true;
        }
        
    }
    return false;
}

fn three_of_a_kind(map : &HashMap<u32,u32>) -> bool {
    println!("{:?}",map);
    if map.len() != 3 {
        return false;
    }
    let values : Vec<u32> = map.values().map(|x| *x).collect();
    if values.contains(&3) {
        return true;
    }
    return false;
}

fn two_pair(map : &HashMap<u32,u32>) -> bool {
    if map.len() != 3 {
        return false;
    }
    let mut values : Vec<u32> = map.values().map(|x| *x).collect();
    values.sort();
    return values[0]==1 && values[1]==2 && values[2]==2;
    
}

fn one_pair(map : &HashMap<u32,u32>) -> bool {
    if map.len() != 4 {
        return false;
    }
    let values : Vec<u32> = map.values().map(|x| *x).collect();
    return values.contains(&2);
}

fn high_card(map : &HashMap<u32,u32>) -> bool {
    return map.len() == 5;
}

fn hand_to_map(hand : &Hand) -> HashMap<u32,u32> {
    let mut map : HashMap<u32,u32> = HashMap::new();
    hand.cards.iter().for_each(|card| {
        if map.contains_key(card) {
            map.insert(*card, map[card]+1);
        }
        else {
            map.insert(*card, 1);
        }
    });
    return map;
}

/*
Score: Five of a kind : 7
Four of a kind : 6
Full house : 5
Three of a kind : 4
Two Pair : 3
One Pair : 2
High card : 1
Nothing : 0
    */    
fn score_hand(hand : &Hand) -> u32 {
    let map = hand_to_map(hand);
    if five_of_a_kind(&map) {
        return 7;
    } else if four_of_a_kind(&map) {
        return 6;
    } else if full_house(&map) {
        return 5;
    } else if three_of_a_kind(&map) {
        return 4;
    } else if two_pair(&map) {
        return 3;
    } else if one_pair(&map) {
        return 2;
    } else if high_card(&map) {
        return 1;
    }


    return 0;

}

fn compare_cards(h1:&Hand, h2:&Hand) -> Ordering{
    
    let score1 = score_hand(h1);
    let score2 = score_hand(h2);

    if score1 > score2 {
        return Ordering::Greater;
    } else if score2 > score1 {
        return Ordering::Less;
    } else {
        for (c1,c2) in h1.cards.iter().zip(h2.cards.iter()) {
            if *c1 > *c2 {
                return Ordering::Greater;
            } else if *c2 > *c1 {
                return Ordering::Less;
            }
        }
    }
    return Ordering::Equal;
}

fn parse(input : &str) -> IResult<&str,Vec<(&str,u64)>> {
    return separated_list1(line_ending, separated_pair(take(5usize), space1, nom::character::complete::u64))(input);
}

fn solve(input : &str) -> usize{
    let parse_result = parse(input).expect("Should parse");
    let mut hands : Vec<Hand> =vec![];
    for (cards_string,bid) in parse_result.1.iter() {
        let mut hand = Hand{cards : [0;5],bid:*bid};
        for (index,c) in cards_string.chars().enumerate() {
            let value = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_digit(10).expect("Should be a digit"),
            };
            hand.cards[index] = value;
        }
        hands.push(hand);
    }
    
    hands.sort_by(compare_cards);
    
    println!("{:?}",hands);
    hands.iter().enumerate().fold(0usize, |acc,(index,card)| {
        acc + (index+1)*card.bid.to_usize()
    })
}


fn main() {
    println!("Hello, world!");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result {}",result);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score() {
        let hand1 = Hand {cards:[2,2,2,2,2],bid:0};
        let hand2 = Hand {cards:[2,3,3,3,2],bid:0};
        assert!(five_of_a_kind(&hand_to_map(&hand1)));
        assert!(full_house(&hand_to_map(&hand2)));
        assert!(!full_house(&hand_to_map(&hand1)));
        let hand3 = Hand {cards:[10,10,10,3,2],bid:0};
        assert!(three_of_a_kind(&hand_to_map(&hand3)));
        assert!(!two_pair(&hand_to_map(&hand3)));
        let hand4 = Hand {cards:[11,11,10,11,11],bid:0};
        assert!(!three_of_a_kind(&hand_to_map(&hand4)));
        assert!(four_of_a_kind(&hand_to_map(&hand4)));

        let hand5 = Hand {cards:[2,3,4,3,2],bid:0};
        assert!(two_pair(&hand_to_map(&hand5)));
        assert!(!one_pair(&hand_to_map(&hand5)));
    }

    #[test]
    fn example() {
        let example = include_str!("../../example.txt");
        assert_eq!(solve(example), 6440);
    }
    
}
