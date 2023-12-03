mod tests;
use std::fs;

pub struct RoundStats {
    red: u16,
    green: u16,
    blue: u16,
}

pub struct Game {
    number: u16,
    rounds: Vec<RoundStats>
}

fn main() {
    let input = fs::read_to_string("input/input.txt")
        .expect("Should have been able to read the file");


    let games = input_string_to_game_vector(&input);

    let mut final_sum : u32 = 0;

    for game in games {
        if game_meets_credentials(&game) {
            final_sum += game.number as u32;
        }
    }

    println!("Final sum of all valid game numbers is {final_sum}!");
}

pub fn game_meets_credentials(game : &Game) -> bool {

    let rounds = &game.rounds;
    for round in rounds {
        if round.red > 12 {
            return false;
        }
        else if round.green > 13 {
            return false;
        }
        else if round.blue > 14 {
            return false;
        }
    }

    true
}

// Somewhat defunct, decided to just pass a game object and skip this middle step.
pub fn get_max_colors(game : &Game) -> RoundStats {

    let mut max : RoundStats = RoundStats { red: 0, green: 0, blue: 0 };

    let rounds = &game.rounds;
    for round in rounds {
        max.red = std::cmp::max(max.red, round.red);
        max.green = std::cmp::max(max.green, round.green);
        max.blue = std::cmp::max(max.blue, round.blue);
    }

    max

}

pub fn input_string_to_game_vector(input: &str) -> Vec<Game> {
    let lines : Vec<&str> = input.split("\n").collect();

    let mut games : Vec<Game> = Vec::new();
    
    for line in lines {
        games.push(game_string_to_object(line));
    }

    games
}

pub fn game_string_to_object(line : &str) -> Game {

    let game_split : Vec<&str> = line.split(":").collect();
    let game_number : Vec<&str> = game_split[0].split(" ").collect();
    let game_number = game_number[1].parse::<u16>().unwrap();

    let round_strings : Vec<&str> = game_split[1].split(";").collect();

    let mut round_stats : Vec<RoundStats> = Vec::new();

    for round_string in round_strings {
        round_stats.push(round_string_to_object(round_string));
    }

    Game {
        number: game_number,
        rounds: round_stats
    }
}

pub fn round_string_to_object(round_string: &str) -> RoundStats {
    let colors: Vec<&str> = round_string.split(",").collect();

    let mut round = RoundStats {
        red: 0,
        green: 0,
        blue: 0
    };

    for col_string in colors {
        let col_string : Vec<&str> = col_string.split(" ").collect();
        let col_count = col_string[1].parse::<u16>().unwrap();

        match col_string[2] {
            "red" => {
                round.red = col_count;
            },
            "green" => {
                round.green = col_count;
            },
            "blue" => {
                round.blue = col_count;
            },
            _ => {
                // Unimportant color we skip this
            }
        }

    }

    round
}