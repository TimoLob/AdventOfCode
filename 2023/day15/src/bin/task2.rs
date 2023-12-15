use std::fmt::Error;

#[derive(Debug)]
struct Box<'a> {
    id: i32,
    lens_slots: Vec<(&'a str, i32)>,
}

fn hash(input: &str) -> Result<i32, Error> {
    if !input.is_ascii() {
        panic!("Input is not valid Ascii");
    }
    let mut hash = 0i32;

    for &c in input.as_bytes().iter() {
        hash += c as i32;
        hash *= 17;
        hash %= 256;
    }
    Ok(hash)
}
// jx=2,nvc=5,xvcn=4,hf-,ggsh=6,tdrm-,ldpk=3,
fn solve(input: &str) -> i32 {
    let steps = input.split(',');
    let mut boxes: Vec<Box> = vec![];
    for i in 0..256 {
        boxes.push(Box {
            lens_slots: vec![],
            id: i,
        });
    }

    for step in steps {
        if step.contains('=') {
            let split = step.split('=').collect::<Vec<&str>>();
            let label = split[0];
            let focal_length = split[1].parse::<i32>().expect("Should parse as num");
            let box_index = hash(label).expect("Label should be hashable") as usize;
            let mut replaced: bool = false;
            for (box_label, focal) in boxes[box_index].lens_slots.iter_mut() {
                if *box_label == label {
                    *focal = focal_length;
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                boxes[box_index].lens_slots.push((label, focal_length));
            }
        } else if step.contains('-') {
            let split = step.split('-').collect::<Vec<&str>>();
            let label = split[0];
            let box_index = hash(label).expect("Label should be hashable") as usize;
            let mut remove_index = boxes[box_index].lens_slots.len();
            for (i, (box_label, _)) in boxes[box_index].lens_slots.iter().enumerate() {
                if *box_label == label {
                    remove_index = i;
                }
            }
            if remove_index < boxes[box_index].lens_slots.len() {
                boxes[box_index].lens_slots.remove(remove_index);
            }
        }
    }
    let mut total = 0;
    for b in boxes.iter() {
        if !b.lens_slots.is_empty() {
            println!("{:?}", b);
            for (lens_index, (_, focal)) in b.lens_slots.iter().enumerate() {
                total += (1 + b.id) * (1 + lens_index as i32) * focal;
            }
        }
    }
    total
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result : {}", result);
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_hash() {
        let input = "HASH";
        assert_eq!(hash(input).unwrap(), 52);

        let inputs = ["rn=1", "cm-", "qp=3", "cm=2"];
        let hashes = [30, 253, 97, 47];
        for (input, expected) in inputs.iter().zip(hashes.iter()) {
            assert_eq!(hash(input).unwrap(), *expected);
        }
    }
}
