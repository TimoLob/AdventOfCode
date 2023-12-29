use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Add,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    length: usize,
    current: Point,
    visited: HashSet<Point>,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Forest,
    Path,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Implement Ord for Path
impl Ord for Path {
    fn cmp(&self, other: &Path) -> std::cmp::Ordering {
        other.length.cmp(&self.length)
    }
}

// Implement PartialOrd for Path
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> (usize, usize, HashMap<Point, Tile>) {
    let mut map = HashMap::new();
    let mut height = 0;
    let mut width = 0;
    for (y, line) in input.lines().enumerate() {
        height = y + 1;
        for (x, c) in line.chars().enumerate() {
            width = x + 1;
            let tile = match c {
                '.' => Tile::Path,
                '#' => Tile::Forest,
                '^' => Tile::Path,
                'v' => Tile::Path,
                '<' => Tile::Path,
                '>' => Tile::Path,
                v => panic!("Unexpected character in input {}", v),
            };
            let point = Point {
                x: x as i64,
                y: y as i64,
            };
            map.insert(point, tile);
        }
    }

    (width, height, map)
}

fn longestPath(map: HashMap<Point, Tile>, width: usize, height: usize) -> usize {
    // Find Start Point
    let mut x = 0;
    let mut start = Point { x: 0, y: 0 };
    loop {
        let tile = map.get(&Point { x, y: start.y });
        if tile.is_none() {
            panic!("Couldn't find start tile");
        }
        let tile = tile.unwrap();
        if *tile == Tile::Path {
            start.x = x;
            break;
        }
        x += 1;
    }
    // Find end point
    let mut x = 0;
    let mut end = Point {
        x: 0,
        y: height as i64 - 1,
    };
    loop {
        let tile = map.get(&Point { x, y: end.y });
        if tile.is_none() {
            panic!("Couldn't find start tile");
        }
        let tile = tile.unwrap();
        if *tile == Tile::Path {
            end.x = x;
            break;
        }
        x += 1;
    }
    println!("Start: {:?}", start);
    println!("End: {:?}", end);

    // Priority Queue of Paths
    let mut pq: BinaryHeap<Path> = std::collections::BinaryHeap::new();
    pq.push(Path {
        length: 0,
        current: start,
        visited: HashSet::new(),
    });
    let mut final_length = 0;
    while let Some(p) = pq.pop() {
        //println!("Path len: {}", p.length);
        if p.current == end {
            final_length = final_length.max(p.length);
        }
        let current = p.current;
        if let Some(tile) = map.get(&p.current) {
            let mut valid_steps = vec![];
            match tile {
                Tile::Forest => continue,
                Tile::Path => {
                    valid_steps.push(Point { x: 0, y: -1 });
                    valid_steps.push(Point { x: 0, y: 1 });
                    valid_steps.push(Point { x: -1, y: 0 });
                    valid_steps.push(Point { x: 1, y: 0 });
                }
                Tile::SlopeUp => valid_steps.push(Point { x: 0, y: -1 }),
                Tile::SlopeDown => valid_steps.push(Point { x: 0, y: 1 }),
                Tile::SlopeLeft => valid_steps.push(Point { x: -1, y: 0 }),
                Tile::SlopeRight => valid_steps.push(Point { x: 1, y: 0 }),
            };
            for step in valid_steps {
                let new_point = current + step;
                if p.visited.contains(&new_point) {
                    continue;
                }
                let mut new_visited = p.visited.clone();
                new_visited.insert(current);
                pq.push(Path {
                    length: p.length + 1,
                    current: new_point,
                    visited: new_visited,
                });
            }
        }
    }
    final_length
}

fn main() {
    let input = include_str!("../../input.txt");
    let (w, h, map) = parse(input);
    let result = longestPath(map, w, h);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn path_ordering() {
        let p1 = Path {
            length: 1,
            current: Point { x: 0, y: 0 },
            visited: HashSet::new(),
        };
        let p2 = Path {
            length: 2,
            current: Point { x: 0, y: 0 },
            visited: HashSet::new(),
        };
        assert!(p1 > p2);
    }

    #[test]
    fn exampletask2() {
        let example = include_str!("../../example.txt");
        let (w, h, map) = parse(example);
        let result = longestPath(map, w, h);
        assert_eq!(result, 154);
    }
}
