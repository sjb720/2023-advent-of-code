mod tests;
use std::fs;

pub enum CharType {
    number,
    symbol,
    space,
}


fn main() {
    let input = fs::read_to_string("input/input.txt")
        .expect("Should have been able to read the file");

    let grid = create_char_grid_from_string(&input);

    
    let sum = grid_sum_valid_codes(&grid);

    println!("Final sum is {sum}");
}

pub fn grid_sum_valid_codes(grid: &Vec<Vec<char>>) -> u32 {

    let mut sum : u32 = 0;

    for row_index in 0..grid.len() {
        sum += row_sum_valid_codes(grid, row_index);
    }

    sum
}

pub fn has_adjacent_symbol(grid: &Vec<Vec<char>>, y: usize, x: usize) -> bool {

    // Check left
    if x > 0 {
        match get_char_type(grid[y][x-1]) {
            CharType::symbol => {
                return true;
            }
            _ => {}
        }

        if y > 0 {
            match get_char_type(grid[y - 1][x-1]) {
                CharType::symbol => {
                    return true;
                }
                _ => {}
            }
        }

        if y < grid.len() - 1 {
            match get_char_type(grid[y + 1][x-1]) {
                CharType::symbol => {
                    return true;
                }
                _ => {}
            }
        }
    }

    // Check right
    if x < grid[0].len() -1 {
        match get_char_type(grid[y][x+1]) {
            CharType::symbol => {
                return true;
            }
            _ => {}
        }

        if y > 0 {
            match get_char_type(grid[y - 1][x+1]) {
                CharType::symbol => {
                    return true;
                }
                _ => {}
            }
        }

        if y < grid.len() - 1 {
            match get_char_type(grid[y + 1][x+1]) {
                CharType::symbol => {
                    return true;
                }
                _ => {}
            }
        }
    }

    // Check up
    if y > 0 {
        match get_char_type(grid[y - 1][x]) {
            CharType::symbol => {
                return true;
            }
            _ => {}
        }
    }

    // Check down
    if y < grid.len() - 1 {
        match get_char_type(grid[y + 1][x]) {
            CharType::symbol => {
                return true;
            }
            _ => {}
        }
    }

    false
}


pub fn row_sum_valid_codes(grid: &Vec<Vec<char>>, y: usize) -> u32 {

    let mut sum : u32 = 0;
    let mut number_is_valid = false;
    let mut current_number = String::from("");
    for x in 0..grid[y].len() {
        let current_char = grid[y][x];

        match get_char_type(current_char) {
            CharType::number => {
                // Check if valid, set true for good. Do not check adjacent if we know we are valid already.
                if !number_is_valid && has_adjacent_symbol(grid, y, x) {
                    number_is_valid = true;
                }
                current_number.push(current_char);
            },
            CharType::space | CharType::symbol => {
                // Reset our number parser
                if number_is_valid {
                    sum += current_number.parse::<u32>().unwrap_or_default();
                }
                current_number = String::from("");
                number_is_valid = false;
            },
        }

    }

    if current_number.len() > 0 {
        // Reset our number parser, we ended a number on a new line.
        if number_is_valid {
            sum += current_number.parse::<u32>().unwrap_or_default();
        }
        current_number = String::from("");
        number_is_valid = false;
    }

    sum
}

pub fn get_char_type(c : char) -> CharType {
    match c {
        '0' | '1' | '2' | '3' | '4'| '5' | '6' | '7'| '8' | '9' => {
            CharType::number
        },
        '.' => {
            CharType::space
        },
        _ => {
            CharType::symbol
        }
    }
}

pub fn create_char_grid_from_string(grid_string:&str) ->Vec<Vec<char>> {

    let lines: Vec<&str> = grid_string.split("\n").collect();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    grid
}



