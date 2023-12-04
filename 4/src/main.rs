mod tests;
use std::{fs, collections::{HashMap, HashSet}};

pub struct Ticket {
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

fn main() {
    let input = fs::read_to_string("input/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = input.split('\n').collect();

    let mut sum : u32 = 0;

    for ticket_string in lines {
        sum += get_score(&ticket_string_to_object(ticket_string));
    }

    println!("Sum of scores is {sum}")

}

pub fn get_score(ticket: &Ticket) -> u32 {
    let mut score = 0;

    let numbers = &ticket.numbers;
    for number in numbers {
        match ticket.winning_numbers.get(number) {
            None => {
                // No match!
            },
            Some(_num) => {
                // Match! We won.
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
    }

    score
}

pub fn ticket_string_to_object(ticket_string: &str) -> Ticket {

    let split: Vec<&str> = ticket_string.split(":").collect();
    let split: Vec<&str> = split[1].split("|").collect();


    Ticket {
        numbers: numbers_to_vec(split[1]),
        winning_numbers: winning_numbers_string_to_hashset(split[0]),
    }
}

pub fn winning_numbers_string_to_hashset(winning_numbers_string: &str) -> HashSet<u32> {
    let mut winning_numbers: HashSet<u32> = HashSet::new();

    let trimmed = winning_numbers_string.trim();
    let trimmed = trimmed.replace("  ", " ");
    let split: Vec<&str> = trimmed.split(" ").collect();

    for num_str in split {
        winning_numbers.insert(num_str.parse::<u32>().unwrap_or_default());
    }

    winning_numbers
}

pub fn numbers_to_vec(numbers_string: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    let trimmed = numbers_string.trim();
    let trimmed = trimmed.replace("  ", " ");
    let split: Vec<&str> = trimmed.split(" ").collect();

    for num_str in split {
        numbers.push(num_str.parse::<u32>().unwrap_or_default());
    }

    numbers
}