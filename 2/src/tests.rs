#[cfg(test)]
mod tests {
    use crate::{game_string_to_object, round_string_to_object, input_string_to_game_vector, get_max_colors, game_meets_credentials};


    #[test]
    fn it_grabs_the_game_number_from_the_string() {
        let game = game_string_to_object("Game 43: 5 blue, 4 red; 10 red, 6 blue; 12 red, 2 blue, 1 green; 7 blue, 12 red, 1 green");
        assert_eq!(game.number, 43);

        let game = game_string_to_object("Game 1: 5 blue, 4 red; 10 red, 6 blue; 12 red, 2 blue, 1 green; 7 blue, 12 red, 1 green");
        assert_eq!(game.number, 1);

        let game = game_string_to_object("Game 1234: 5 blue, 4 red; 10 red, 6 blue; 12 red, 2 blue, 1 green; 7 blue, 12 red, 1 green");
        assert_eq!(game.number, 1234);
    }

    #[test]
    fn it_converts_a_round_string_to_an_object() {
        let round = round_string_to_object(" 5 blue, 4 red");
        assert_eq!(round.red, 4);
        assert_eq!(round.blue, 5);
        assert_eq!(round.green, 0);

        let round = round_string_to_object(" 10 green, 50 blue, 14 red");
        assert_eq!(round.red, 14);
        assert_eq!(round.blue, 50);
        assert_eq!(round.green, 10);

        let round = round_string_to_object(" 1 blue");
        assert_eq!(round.red, 0);
        assert_eq!(round.blue, 1);
        assert_eq!(round.green, 0);

    }

    #[test]
    fn it_converts_a_game_string_into_an_object_completely() {
        let game = game_string_to_object("Game 43: 5 blue, 4 red; 10 red, 6 blue; 12 red, 2 blue, 1 green; 7 blue, 12 red, 1 green");
        assert_eq!(game.number, 43);
        assert_eq!(game.rounds.len(), 4);
        
        assert_eq!(game.rounds[0].blue, 5);
        assert_eq!(game.rounds[0].red, 4);
        
        assert_eq!(game.rounds[3].blue, 7);
        assert_eq!(game.rounds[3].green, 1);
        assert_eq!(game.rounds[3].red, 12);

    }

    #[test]
    fn it_converts_a_multiline_input_into_a_game_vector() {
        let games = input_string_to_game_vector("Game 40: 4 green, 12 blue; 5 red, 13 blue, 1 green; 4 green, 7 red; 7 blue, 2 green\nGame 41: 3 red, 1 green; 10 green, 4 blue, 5 red; 8 blue, 5 red");
        assert_eq!(games[0].number, 40);
        assert_eq!(games[0].rounds.len(), 4);

        assert_eq!(games[1].number, 41);
        assert_eq!(games[1].rounds.len(), 3);

    }

    #[test]
    fn it_converts_a_game_string_and_gets_max_colors() {
        let game = game_string_to_object("Game 40: 4 green, 12 blue; 5 red, 13 blue, 1 green; 69 green, 7 red; 7 blue, 2 green");
        
        let max = get_max_colors(&game);

        assert_eq!(max.red, 7);
        assert_eq!(max.blue, 13);
        assert_eq!(max.green, 69);
    }

    #[test]
    fn it_correctly_judges_if_game_meets_credentials() {
        let game = game_string_to_object("Game 40: 4 green, 12 blue; 5 red, 13 blue, 1 green; 69 green, 7 red; 7 blue, 2 green");
        assert_eq!(game_meets_credentials(&game), false);

        let game = game_string_to_object("Game 12: 4 green, 2 blue; 5 red, 4 blue, 1 green; 5 green, 7 red; 7 blue, 2 green");
        assert_eq!(game_meets_credentials(&game), true);

        let game = game_string_to_object("Game 15: 14 green, 2 blue; 5 red, 15 blue, 1 green; 5 green, 13 red; 7 blue, 2 green");
        assert_eq!(game_meets_credentials(&game), false);

        let game = game_string_to_object("Game 15: 13 green, 2 blue; 5 red, 14 blue, 1 green; 5 green, 12 red; 7 blue, 2 green");
        assert_eq!(game_meets_credentials(&game), true);
    }

}