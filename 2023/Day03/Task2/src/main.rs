use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn is_number(symbol: u8) -> bool {
    (0x30..0x3A).contains(&symbol)
}

fn get_number(line: &mut [u8], x: usize) -> i32 {
    // If one digit is found, this function finds the whole number and replaces it with ...
    let mut pos = x;
    //println!("{} {}", pos, char::from(line[pos]));

    //println!("{}", std::str::from_utf8(line).unwrap());
    while pos<line.len()&&is_number(line[pos])  {
        pos += 1;
    }
    pos -= 1; // At last digit
    let mut number: i32 = i32::from(line[pos] - 0x30);
    line[pos] = 0x2E;
    pos -= 1;
    let mut exp = 1;
    const BASE: i32 = 10;
    while is_number(line[pos]) {
        number += i32::from(line[pos] - 0x30) * BASE.pow(exp);
        line[pos] = 0x2E;
        if pos==0 {
            break;
        }
        pos -= 1;
        exp += 1;
    }
    println!("Number {}",number);
    number
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let file_name = &args[1];
    let content = fs::read_to_string(file_name).expect("Error reading file");
    // println!("{}",content);
    let lines: Vec<&str> = content.lines().collect();
    println!("{}", lines.len());

    let mut array: Vec<Vec<u8>> = vec![];

    for line in lines {
        let mut bytes: Vec<u8> = vec![];

        for byte_char in line.bytes() {
            bytes.push(byte_char);
        }
        array.push(bytes);
    }

    let mut total = 0;
    for y in 0..array.len() {
        for x in 0..array[y].len() {
            let symbol = array[y][x];
            let mut part_numbers:Vec<i32> = vec![];
            if char::from(symbol) == '*' {
                if y>0 {
                    println!("{}",std::str::from_utf8(array[y-1].as_slice()).unwrap());
                }
                println!("{}",std::str::from_utf8(array[y].as_slice()).unwrap());
                if y<array.len()-1 {
                    println!("{}",std::str::from_utf8(array[y+1].as_slice()).unwrap())
                }
                println!("Found symbol {} at position {} Current Total: {}",char::from(array[y][x]), x, total);

                // Check the surrounding for numbers
                if y > 0 && x > 0 && is_number(array[y - 1][x - 1]) {
                    part_numbers.push(get_number(array[y - 1].as_mut_slice(), x - 1));
                }
                if y > 0 && is_number(array[y - 1][x]) {
                    part_numbers.push(get_number(array[y - 1].as_mut_slice(), x));
                }
                if y > 0 && x < array[y].len() - 1 && is_number(array[y - 1][x + 1]) {
                    part_numbers.push(get_number(array[y - 1].as_mut_slice(), x + 1));
                }
                if x>0 && is_number(array[y][x-1]) {
                    part_numbers.push(get_number(array[y].as_mut_slice(), x-1));
                }
                if x < array[y].len() && is_number(array[y][x+1]) {
                    part_numbers.push(get_number(array[y].as_mut_slice(), x+1));
                }

                if y < array.len()-1 && x > 0 && is_number(array[y + 1][x - 1]) {
                    part_numbers.push(get_number(array[y + 1].as_mut_slice(), x - 1));
                }
                if y < array.len()-1 && is_number(array[y + 1][x]) {
                    part_numbers.push(get_number(array[y + 1].as_mut_slice(), x));
                }
                if y < array.len()-1 && x < array[y].len() - 1 && is_number(array[y + 1][x + 1]) {
                    part_numbers.push(get_number(array[y + 1].as_mut_slice(), x + 1));
                }
                if part_numbers.len() == 2 {
                    total += part_numbers[0]*part_numbers[1];
                }
            }
        }
        
    }
    println!("Final Total: {}", total);

    let mut file = File::create("output.txt").unwrap();
    for line in array {
        file.write_all(line.as_slice());
        let buf = [b'\n'];
        file.write(&buf);
    }

}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_is_number() {
        assert!(is_number(b'1'));
    }
    #[test]
    fn all_numbers() {
        assert!(is_number(b'0'));
        assert!(is_number(b'1'));
        assert!(is_number(b'2'));
        assert!(is_number(b'3'));
        assert!(is_number(b'4'));
        assert!(is_number(b'5'));
        assert!(is_number(b'6'));
        assert!(is_number(b'7'));
        assert!(is_number(b'8'));
        assert!(is_number(b'9'));
    }

    #[test]
    fn not_numbers() {
        assert!(!is_number(0x2F));
        assert!(!is_number(0x3A));
        assert!(!is_number(b'.'));
        assert!(!is_number(b'9'+1));
        assert!(!is_number(b'0'-1));
    }
}
