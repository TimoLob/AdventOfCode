use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::line_ending, multi::separated_list1,
    sequence::separated_pair, IResult,
};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    lower: Point,
    upper: Point,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, (y, z))) = separated_pair(
        nom::character::complete::digit1,
        tag(","),
        separated_pair(
            nom::character::complete::digit1,
            tag(","),
            nom::character::complete::digit1,
        ),
    )(input)?;
    let x = x.parse::<i32>().unwrap();
    let y = y.parse::<i32>().unwrap();
    let z = z.parse::<i32>().unwrap();

    Ok((input, Point { x, y, z }))
}

fn parse_block(input: &str) -> IResult<&str, Block> {
    let (input, (point1, point2)) = separated_pair(parse_point, tag("~"), parse_point)(input)?;
    // Swap coordinates such that upper always contains the largest values in each dimension
    let x_lower = point1.x.min(point2.x);
    let x_upper = point1.x.max(point2.x);
    let y_lower = point1.y.min(point2.y);
    let y_upper = point1.y.max(point2.y);
    let z_lower = point1.z.min(point2.z);
    let z_upper = point1.z.max(point2.z);
    let lower = Point {
        x: x_lower,
        y: y_lower,
        z: z_lower,
    };
    let upper = Point {
        x: x_upper,
        y: y_upper,
        z: z_upper,
    };
    Ok((input, Block { lower, upper }))
}

fn parse(input: &str) -> IResult<&str, Vec<Block>> {
    let (input, mut blocks) = separated_list1(line_ending, parse_block)(input)?;
    // Sort by lower z
    blocks.sort_by(|a, b| a.lower.z.cmp(&b.lower.z));
    Ok((input, blocks))
}

fn a_under_b(a: &Block, b: &Block) -> bool {
    //println!("a: {:?}, b: {:?}", a, b);
    !(a.upper.x < b.lower.x
        || a.lower.x > b.upper.x
        || a.upper.y < b.lower.y
        || a.lower.y > b.upper.y)
}
// How many blocks are under this block, supporting it.
fn count_supporting_blocks(block: &Block, blocks_by_z: &Vec<Vec<Block>>) -> i32 {
    let mut count = 0;
    let z = block.lower.z - 1;
    println!("Block: {:?}", block);
    for support in blocks_by_z[z as usize].iter() {
        if a_under_b(support, block) {
            count += 1;
        }
    }
    count
}
// Count other blocks only supported by this block.
fn count_blocks_supported_by(block: &Block, blocks_by_z: &Vec<Vec<Block>>) -> i32 {
    let mut count = 0;
    let z = block.upper.z + 1;
    if z as usize >= blocks_by_z.len() {
        return 0;
    }
    for support in blocks_by_z[z as usize].iter() {
        if a_under_b(block, support) {
            //count += 1;
            let supports_of_support = count_supporting_blocks(support, blocks_by_z);
            // println!("supports_of_support: {}", supports_of_support);
            if supports_of_support <= 1 {
                count += 1;
            }
        }
    }
    count
}

fn solve(blocks: &Vec<Block>) -> i32 {
    let mut count = 0;
    let max_x = blocks.iter().map(|b| b.upper.x).max().unwrap();
    let max_y = blocks.iter().map(|b| b.upper.y).max().unwrap();
    println!("max_x: {}, max_y: {}", max_x, max_y);
    //println!("blocks: {:?}", blocks);

    // Lower blocks until they reach the gound at z=0 or land on a block under them
    let blocks = blocks.clone();
    // Fall into tower
    let mut tower: Vec<Block> = Vec::new();
    for block in blocks.iter() {
        if block.lower.z == 1 {
            tower.push(*block);
            continue;
        }
        let mut block = *block;
        let mut z: i32 = 1;
        for tower_block in tower.iter() {
            // println!("tower_block: {:?}", tower_block);
            if a_under_b(tower_block, &block) {
                // println!("tower_block: {:?} is under block: {:?}", tower_block, block);
                z = tower_block.upper.z + 1;
                continue;
            }
        }
        //println!("Block: {:?}, z: {}", block, z);
        let block_height = block.upper.z - block.lower.z;
        block.lower.z = z;
        block.upper.z = z + block_height;
        tower.push(block);
    }

    // Build structure
    let max_z = tower.iter().map(|b| b.upper.z).max().unwrap() as usize;
    println!("max_z: {}", max_z);
    let mut blocks_by_z: Vec<Vec<Block>> = Vec::new();

    for z in 0..max_z {
        let mut blocks_at_z: Vec<Block> = Vec::new();
        for block in tower.iter() {
            if block.lower.z <= z as i32 && block.upper.z >= z as i32 {
                blocks_at_z.push(*block);
            }
        }
        blocks_by_z.push(blocks_at_z);
    }

    for (z, blocks) in blocks_by_z.iter().enumerate() {
        println!("z: {}, blocks: {:?}", z, blocks);
    }

    for block in tower.iter() {
        let supports = count_blocks_supported_by(block, &blocks_by_z);

        println!("block: {:?}, supports: {}", block, supports);
        if supports == 0 {
            count += 1;
        }
    }

    //println!("tower: {:?}", tower);
    count
}

fn main() {
    let input = include_str!("../../input.txt");
    let (input, blocks) = parse(input).expect("");
    let result = solve(&blocks);
    println!("{}", result);
}
