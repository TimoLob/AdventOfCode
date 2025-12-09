use std::{collections::HashMap, ops::RangeInclusive};

use glam::I64Vec2;
use memoize::memoize;
use rayon::prelude::*;
fn parse(input : &str) -> Vec<I64Vec2> {
    input.lines().map(|line| { 
        let split = line.split_once(',').unwrap();
        I64Vec2::new(split.0.parse().unwrap(), split.1.parse().unwrap())

    } ).collect()
}

fn rect_area(a: &I64Vec2, b: &I64Vec2) -> i64 {
    ((a.x-b.x).abs()+1) * ((a.y - b.y).abs()+1)
}


pub fn part1(input: &str) -> String {
    let input = input.trim();
    let coords: Vec<I64Vec2> = parse(input);
    let mut largest_area = 0;

    for i in 0..coords.len() {
        for j in (i+1)..coords.len() {
            let a = coords[i];
            let b = coords[j];
            let area = rect_area(&a, &b);

            if area > largest_area {
                largest_area = area;
            }
        }
    }
    largest_area.to_string()
}


/// Returns true if `p` is inside (or on the boundary of) the polygon `poly`.
/// `poly` is a slice of vertices in order (clockwise or counter-clockwise),
/// representing a non-self-intersecting polygon.
#[memoize(Ignore:poly)]
pub fn point_in_polygon(p: I64Vec2, poly: &[I64Vec2]) -> bool {
    let n = poly.len();
    if n < 3 {
        return false;
    }

    // First, check if we're exactly on an edge.
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        if point_on_segment(p, a, b) {
            return true;
        }
    }

    // Ray casting: horizontal ray to +x
    let mut inside = false;
    let px = p.x as f64;
    let py = p.y as f64;

    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];

        let (x1, y1) = (a.x as f64, a.y as f64);
        let (x2, y2) = (b.x as f64, b.y as f64);

        // Check if edge (a,b) crosses a horizontal line at y = py
        let intersects = ((y1 > py) != (y2 > py)) &&
            (px < (x2 - x1) * (py - y1) / (y2 - y1) + x1);

        if intersects {
            inside = !inside;
        }
    }

    inside
}

/// Returns true if point p lies exactly on the line segment ab.
fn point_on_segment(p: I64Vec2, a: I64Vec2, b: I64Vec2) -> bool {
    let ap = p - a;
    let ab = b - a;

    // Cross product == 0 → collinear
    let cross = ap.x as i64 * ab.y as i64 - ap.y as i64 * ab.x as i64;
    if cross != 0 {
        return false;
    }

    // Check that p is between a and b (dot product and length)
    let dot = ap.x as i64 * ab.x as i64 + ap.y as i64 * ab.y as i64;
    if dot < 0 {
        return false;
    }

    let ab_len_sq = ab.x as i64 * ab.x as i64 + ab.y as i64 * ab.y as i64;
    if dot > ab_len_sq {
        return false;
    }

    true
}



pub fn part2(input: &str) -> String {
    let input = input.trim();
    let mut coords: Vec<I64Vec2> = parse(input);

    let min_x = coords.iter().map(|v|v.x).min().unwrap();
    let min_y = coords.iter().map(|v|v.y).min().unwrap();
    for coord in coords.iter_mut() {
        coord.x -= min_x;
        coord.y -= min_y;
    }
    let min_x = coords.iter().map(|v|v.x).min().unwrap();
    let min_y = coords.iter().map(|v|v.y).min().unwrap();
    let max_x: i64 = coords.iter().map(|v|v.x).max().unwrap();
    let max_y = coords.iter().map(|v|v.y).max().unwrap();
    let mut used_x_coords = vec![false;(max_x+1) as usize];
    let mut used_y_coords = vec![false;(max_y+1) as usize];
    for v in coords.iter() {
        used_x_coords[v.x as usize] = true;
        used_y_coords[v.y as usize] = true;
    }

    let mut largest_area = 0;
    for i in 0..coords.len() {
        println!("{}/{}",i,coords.len());
        for j in (i+1)..coords.len() {
            println!(":{}/{}",j,coords.len());

            let a = coords[i];
            let b = coords[j];
            let area = rect_area(&a, &b);

            if area < largest_area {
                continue;
            }

            let all_inside = (a.x.min(b.x)..=(a.x.max(b.x))).all(|x| {

                (a.y.min(b.y)..=(a.y.max(b.y))).all(|y| {
                    if used_x_coords[x as usize] && used_y_coords[y as usize] {
                        point_in_polygon(I64Vec2 { x, y }, &coords)
                    }
                    else {
                        true
                    }
                })

            });
            if all_inside {
                largest_area = area;
            }
        }
    }
    
    largest_area.to_string() 
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part1(&input);
        assert_eq!(result, "50"); // Replace with the actual expected result
    }
    #[test]
    fn test_example_part2() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        let result = part2(&input);
        assert_eq!(result, "24"); // Replace with the actual expected result
    }

    #[test]
    fn test_memoization() {
        let input = fs::read_to_string("example.txt").expect("Failed to read example.txt");

        let coords = parse(&input);
        point_in_polygon(coords[0], &coords);
        point_in_polygon(coords[0], &coords);
        point_in_polygon(coords[1], &coords);
        point_in_polygon(coords[0], &coords);

        point_in_polygon(coords[0], &coords);

        point_in_polygon(coords[0], &coords);

    }
}
