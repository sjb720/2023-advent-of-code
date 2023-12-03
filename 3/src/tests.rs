#[cfg(test)]
mod tests {
    use crate::{create_char_grid_from_string, row_sum_valid_codes, has_adjacent_symbol};



    #[test]
    fn it_converts_an_input_string_into_a_char_grid () {

        let input = ".....123...44....\n...#.1*3...4@....\n...........69....";
        let grid = create_char_grid_from_string(&input);
        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0].len(), 17);
        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0][5], '1');
        assert_eq!(grid[1][6], '*');
    }

    
    #[test]
    fn it_sums_a_row_of_valid_numbers_where_symbols_are_adjacent_on_row () {

        let input = "....*123..*44....\n...#.1*3...4@....\n...........69*...";
        let grid: Vec<Vec<char>> = create_char_grid_from_string(&input);
        
        assert_eq!(row_sum_valid_codes(&grid, 0), 123+44);
        assert_eq!(row_sum_valid_codes(&grid, 1), 1+3+4);
        assert_eq!(row_sum_valid_codes(&grid, 2), 69);
        
    }

    #[test]
    fn it_sums_a_row_of_valid_numbers_where_symbols_are_adjacent_below_and_above () {

        let input = "..12..34..54\n.*.........$";
        let grid: Vec<Vec<char>> = create_char_grid_from_string(&input);
        assert_eq!(row_sum_valid_codes(&grid, 0), 12+54);
        
    }

    #[test]
    fn it_finds_adjacent_symbols () {
        let input = ".....\n.....\n..4..\n.....\n.....";
        let grid: Vec<Vec<char>> = create_char_grid_from_string(&input);
        assert_eq!(has_adjacent_symbol(&grid, 2, 2), false);

        let input = ".....\n...&.\n..4..\n.....\n.....";
        let grid: Vec<Vec<char>> = create_char_grid_from_string(&input);
        assert_eq!(has_adjacent_symbol(&grid, 2, 2), true);

        let input = ".....\n.....\n..4..\n.%...\n.....";
        let grid: Vec<Vec<char>> = create_char_grid_from_string(&input);
        assert_eq!(has_adjacent_symbol(&grid, 2, 2), true);

        let input = ".....\n.....\n..4..\n.....\n.....";
        let grid: Vec<Vec<char>> = create_char_grid_from_string(&input);
        assert_eq!(has_adjacent_symbol(&grid, 2, 2), false);

        let input = ".....\n...&.\n..4..\n.....\n.....";
        let grid: Vec<Vec<char>> = create_char_grid_from_string(&input);
        assert_eq!(has_adjacent_symbol(&grid, 2, 2), true);

        let input = ".....\n.....\n...4*";
        let grid: Vec<Vec<char>> = create_char_grid_from_string(&input);
        assert_eq!(has_adjacent_symbol(&grid, 2, 3), true);

        let input = "&4...\n.....\n.....";
        let grid: Vec<Vec<char>> = create_char_grid_from_string(&input);
        assert_eq!(has_adjacent_symbol(&grid, 0, 1), true);
    }


}