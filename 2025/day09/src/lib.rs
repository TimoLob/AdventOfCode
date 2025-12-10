use glam::I64Vec2;
use memoize::memoize;
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

/// Compress coordinate space
/// 1. Normalize coordinates such that the minimum is (0/0)
/// 2. Find all used X and Y coordinates
/// 3. Map coordinates to their index of use
/// 
/// E.g. if X coordinates 1,3, 6,7 are used those will be mapped as
/// 1 => 0
/// 3 => 1
/// 6 => 2
/// 7 => 3
/// 
/// This keeps relative ordering but distorts distances. We will use this compressed space to test whether points are in the polygon or not
fn compress_vec(coords: &Vec<I64Vec2>) -> Vec<I64Vec2> {
    let mut coords = coords.clone();
    let min_x = coords.iter().map(|v|v.x).min().unwrap();
    let min_y = coords.iter().map(|v|v.y).min().unwrap();
    for coord in coords.iter_mut() {
        coord.x -= min_x;
        coord.y -= min_y;
    }
    let max_x: i64 = coords.iter().map(|v|v.x).max().unwrap();
    let max_y = coords.iter().map(|v|v.y).max().unwrap();
    let mut used_x_coords = vec![false;(max_x+1) as usize];
    let mut used_y_coords = vec![false;(max_y+1) as usize];
    for v in coords.iter() {
        used_x_coords[v.x as usize] = true;
        used_y_coords[v.y as usize] = true;
    }
    let x_coordinate_map = used_x_coords.iter().enumerate()
    .filter(|(_idx,b)| **b).enumerate().map(|(_i1, (i2,_))| i2).collect::<Vec<usize>>();
    let y_coordinate_map = used_y_coords.iter().enumerate()
    .filter(|(_idx,b)| **b).enumerate().map(|(_i1, (i2,_))| i2).collect::<Vec<usize>>();


    for v in coords.iter_mut() {
        v.x = x_coordinate_map.iter().position(|&x| x as i64 == v.x).unwrap() as i64;
        v.y = y_coordinate_map.iter().position(|&y| y as i64 == v.y).unwrap() as i64;

    }
    coords
}


pub fn part2(input: &str) -> String {
    let input = input.trim();
    let coords: Vec<I64Vec2> = parse(input);

    let comprossed_coords = compress_vec(&coords);

    let mut largest_area = 0;

    for i in 0..coords.len() {
        for j in (i+1)..coords.len() {

            let ca = comprossed_coords[i];
            let cb = comprossed_coords[j];
            let area = rect_area(&coords[i], &coords[j]); // Use real coords for area calculation

            if area < largest_area { // If rectangle spanned by corners isn't a canditate, don't bother checking if it is legal
                continue;
            }
            // Check all points in the rectangle
            let all_inside = (ca.x.min(cb.x)..=(ca.x.max(cb.x))).all(|x| {
                (ca.y.min(cb.y)..=(ca.y.max(cb.y))).all(|y| {
                    point_in_polygon(I64Vec2 { x, y }, &comprossed_coords)
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
