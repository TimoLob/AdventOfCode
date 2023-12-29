use std::collections::{BinaryHeap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    number::{self, complete::double},
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Clone, Copy, Debug)]
struct Line {
    start: Point,
    slope: [f64; 3],
}

#[derive(Clone, Copy, Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Line {
    fn to_line_segment(&self, lower: f64, upper: f64) -> Option<LineSegment> {
        // Bounding Box given by lower and upper in X and Y direction. Find the line segment that is inside of this bounding box
        let mut intersections: Vec<Point> = Vec::new();
        let start = self.start;
        // Intersection with upper
        let dx = (upper - start.x) / self.slope[0];
        let dy = (upper - start.y) / self.slope[1];
        if dx > 0.0 {
            let y = start.y + dx * self.slope[1];
            if y >= lower && y <= upper {
                intersections.push(Point {
                    x: upper,
                    y,
                    z: start.z + dx * self.slope[2],
                });
            }
        }
        if dy > 0.0 {
            let x = start.x + dy * self.slope[0];
            if x >= lower && x <= upper {
                intersections.push(Point {
                    x,
                    y: upper,
                    z: start.z + dy * self.slope[2],
                });
            }
        }
        // Intersection with lower
        let dx = (lower - start.x) / self.slope[0];
        let dy = (lower - start.y) / self.slope[1];
        if dx > 0.0 {
            let y = start.y + dx * self.slope[1];
            if y >= lower && y <= upper {
                intersections.push(Point {
                    x: lower,
                    y,
                    z: start.z + dx * self.slope[2],
                });
            }
        }
        if dy > 0.0 {
            let x = start.x + dy * self.slope[0];
            if x >= lower && x <= upper {
                intersections.push(Point {
                    x,
                    y: lower,
                    z: start.z + dy * self.slope[2],
                });
            }
        }
        println!("Start: {:?}", start);
        println!("Slope: {:?}", self.slope);
        println!("Intersections: {:?}", intersections);

        if intersections.len() == 1 {
            let end = intersections[0];
            if start.y < end.y {
                return Some(LineSegment { start, end });
            }
            return Some(LineSegment {
                start: end,
                end: start,
            });
        }
        if intersections.len() == 2 {
            let start = intersections[0];
            let end = intersections[1];
            if start.y < end.y {
                return Some(LineSegment { start, end });
            }
            return Some(LineSegment {
                start: end,
                end: start,
            });
        }

        None
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let lines = parse(input, 200000000000000.0, 400000000000000.0);
    println!("Intersections: {}", find_intersections(lines));
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, (xyz, dir)) = separated_pair(
        separated_list1(preceded(tag(","), space1), double),
        tag(" @ "),
        separated_list1(preceded(tag(","), space1), double),
    )(input)?;

    let (x, y, z) = (xyz[0], xyz[1], xyz[2]);
    let (dx, dy, dz) = (dir[0], dir[1], dir[2]);
    Ok((
        input,
        Line {
            start: Point { x, y, z },
            slope: [dx, dy, dz],
        },
    ))
}

// Lower and upper are bounds to the X axis
fn parse(input: &str, lower: f64, upper: f64) -> Vec<LineSegment> {
    let (input, lines) = separated_list1(line_ending, parse_line)(input).unwrap();
    lines
        .iter()
        .filter_map(|line| line.to_line_segment(lower, upper))
        .collect()
}

enum Event {
    Start(Point, LineSegment),
    End(Point, LineSegment),
    Intersection(Point, LineSegment, LineSegment),
}

fn find_intersections(lines: Vec<LineSegment>) -> u32 {
    println!("Lines: {:#?}", lines);
    let mut events: BinaryHeap<Event> = BinaryHeap::new();

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let example = include_str!("../../example.txt");
        let lines = parse(example, 7f64, 27f64);
        assert_eq!(find_intersections(lines), 2);
    }
}
