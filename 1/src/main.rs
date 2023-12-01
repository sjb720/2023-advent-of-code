mod tests;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt")
        .expect("Should have been able to read the file");

    let codes = split_by_new_line(&input);

    let code_sum = sum_codes(codes);

    println!("The final sum is: {code_sum}")
}

pub fn split_by_new_line(content : &String) -> Vec<&str> {
    content.split("\n").collect()
}


fn sum_codes(codes : Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    for code in codes {
        sum += calculate_code(code) as u32;
    }
    sum
}

pub fn calculate_code(code : &str) -> u8 {
    (10 * get_first_number(code)) + get_last_number(code)
}

pub fn get_first_number(code : &str) -> u8 {
    let bytes = code.as_bytes();
    for (_i, &item) in bytes.iter().enumerate() {
        if is_number(item) {
            return item - 48;
        }
    }

    0
}

pub fn get_last_number(code : &str) -> u8 {
    let bytes = code.as_bytes();
    for (_i, &item) in bytes.iter().rev().enumerate() {
        if is_number(item) {
            return item - 48;
        }
    }

    0
}

pub fn is_number(byte : u8) -> bool {

    match byte {
        b'0' => true,
        b'1' => true,
        b'2' => true,
        b'3' => true,
        b'4' => true,
        b'5' => true,
        b'6' => true,
        b'7' => true,
        b'8' => true,
        b'9' => true,
        _ => false
    }

}