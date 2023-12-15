use std::fmt::Error;

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

fn main() {
    let input = include_str!("../../input.txt");
    let result = input.split(',').map(|x| hash(x).unwrap()).sum::<i32>();
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
