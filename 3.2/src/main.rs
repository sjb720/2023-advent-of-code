mod tests;
use std::{fs, collections::{HashMap, HashSet}};

pub enum CharType {
    number,
    symbol,
    space,
    gear
}


fn main() {
    let input = fs::read_to_string("input/input.txt")
        .expect("Should have been able to read the file");

    let grid = create_char_grid_from_string(&input);

    let mut gear_nums: HashMap<String, Vec<u16>> = HashMap::new();
    
    grid_collect_gear_adj_numbers(&grid, &mut gear_nums);

    let mut sum = 0;

    for (_key, val) in gear_nums {
        if val.len() == 2 {
            sum += val[0] as u32 * val[1] as u32;
        }
    }

    println!("Final sum is {sum}");
}

pub fn grid_collect_gear_adj_numbers(grid: &Vec<Vec<char>>, gear_nums : 
    &mut HashMap<String, Vec<u16>>) {


    for row_index in 0..grid.len() {
        collect_gear_adj_numbers(grid, row_index, gear_nums);
    }
}

pub fn get_adjacent_gears(grid: &Vec<Vec<char>>, y: usize, x: usize) -> HashSet<String> {

    let mut gears = HashSet::new();

    // Check left
    if x > 0 {
        match get_char_type(grid[y][x-1]) {
            CharType::gear => {
                gears.insert(format!("{},{}", y, x-1));
            }
            _ => {}
        }

        if y > 0 {
            match get_char_type(grid[y - 1][x-1]) {
                CharType::gear => {
                    gears.insert(format!("{},{}", y-1, x-1));
                }
                _ => {}
            }
        }

        if y < grid.len() - 1 {
            match get_char_type(grid[y + 1][x-1]) {
                CharType::gear => {
                    gears.insert(format!("{},{}", y+1, x-1));
                }
                _ => {}
            }
        }
    }

    // Check right
    if x < grid[0].len() -1 {
        match get_char_type(grid[y][x+1]) {
            CharType::gear => {
                gears.insert(format!("{},{}", y, x+1));
            }
            _ => {}
        }

        if y > 0 {
            match get_char_type(grid[y - 1][x+1]) {
                CharType::gear => {
                    gears.insert(format!("{},{}", y-1, x+1));
                }
                _ => {}
            }
        }

        if y < grid.len() - 1 {
            match get_char_type(grid[y + 1][x+1]) {
                CharType::gear => {
                    gears.insert(format!("{},{}", y+1, x+1));
                }
                _ => {}
            }
        }
    }

    // Check up
    if y > 0 {
        match get_char_type(grid[y - 1][x]) {
            CharType::gear => {
                gears.insert(format!("{},{}", y-1, x));
            }
            _ => {}
        }
    }

    // Check down
    if y < grid.len() - 1 {
        match get_char_type(grid[y + 1][x]) {
            CharType::gear => {
                gears.insert(format!("{},{}", y+1, x));
            }
            _ => {}
        }
    }

    gears
}


pub fn collect_gear_adj_numbers(grid: &Vec<Vec<char>>, y: usize, gear_nums:  &mut HashMap<String, Vec<u16>>)  {

    let mut current_number = String::from("");
    let mut adj_gears : HashSet<String> = HashSet::new();
    for x in 0..grid[y].len() {
        let current_char = grid[y][x];

        match get_char_type(current_char) {
            CharType::number => {
                // Find adj gears and insert them into our master set
                adj_gears.extend(get_adjacent_gears(grid, y, x));
                current_number.push(current_char);
            },
            CharType::space | CharType::symbol | CharType::gear => {
                // Reset our number parser
                let num = current_number.parse::<u16>().unwrap_or_default();
                for gear in adj_gears {
                    match gear_nums.get(&gear) {
                        None => {
                            gear_nums.insert(gear, vec![num]);
                        },
                        Some(arr)=> {
                            let mut t = arr.clone();
                            t.push(num);
                            gear_nums.insert(gear, t);
                        }
                    }
                    
                }

                // Reset vars
                adj_gears = HashSet::new();
                current_number = String::from("");
            },
        }

    }

    if current_number.len() > 0 {
        // Reset our number parser, we ended a number on a new line.
        let num = current_number.parse::<u16>().unwrap_or_default();
            for gear in adj_gears {
                match gear_nums.get(&gear) {
                    None => {
                        gear_nums.insert(gear, vec![num]);
                    },
                    Some(arr)=> {
                        let mut t = arr.clone();
                        t.push(num);
                        gear_nums.insert(gear, t);
                    }
                }
                
            }
    }

}

pub fn get_char_type(c : char) -> CharType {
    match c {
        '0' | '1' | '2' | '3' | '4'| '5' | '6' | '7'| '8' | '9' => {
            CharType::number
        },
        '.' => {
            CharType::space
        },
        '*' => {
            CharType::gear
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



