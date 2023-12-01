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
    for (i, &item) in bytes.iter().enumerate() {
        if is_number(item) {
            return item - 48;
        } else 
        {
        match get_number_from_start_of_string(&code[i..]) {
            Err(_err) => {},
            Ok(num) => {return num;} 
        }
    }
    }

    0
}

pub fn get_number_from_start_of_string(slice: &str) -> Result<u8,String> {

    match slice.bytes().next() {
        None => {
            return Err("Cannot be a number. No bytes found in string.".to_owned())
        },
        Some(byte) => {
            match byte {
                b'o' => {
                    if slice.starts_with("one") {
                        return Ok(1);
                    }
                },
                b't' => {
                    if slice.starts_with("two") {
                        return Ok(2);
                    } else if slice.starts_with("three") {
                        return Ok(3);
                    } 
                },
                b'f' => {
                    if slice.starts_with("four") {
                        return Ok(4);
                    }
                    else if slice.starts_with("five") {
                        return Ok(5);
                    }
                },
                b's' => {
                    if slice.starts_with("six") {
                        return Ok(6);
                    }
                    else if slice.starts_with("seven") {
                        return Ok(7);
                    }
                },
                b'e' => {
                    if slice.starts_with("eight") {
                        return Ok(8);
                    }
                }
                b'n' => {
                    if slice.starts_with("nine") {
                        return Ok(9);
                    }
                }
                _ => {
                    return Err("Cannot be a number, no valid starting char.".to_owned());
                }

            }
        }
    }

    Err("There is no number at the start of this string.".to_owned())
}

pub fn get_last_number(code : &str) -> u8 {
    let bytes = code.as_bytes();
    let byte_len = bytes.len();
    for (i, &item) in bytes.iter().rev().enumerate() {
        if is_number(item) {
            return item - 48;
        }else 
        {
        match get_number_from_start_of_string(&code[(byte_len - i - 1)..]) {
            Err(_err) => {},
            Ok(num) => {return num;} 
        }
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