#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{create_char_grid_from_string, get_adjacent_gears, grid_collect_gear_adj_numbers};



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
    fn it_finds_all_gears_adjacent_to_number () {

        let input = ".....\n..*..\n..5*.\n.*...\n.....";
        let grid = create_char_grid_from_string(&input);
        

        let set = get_adjacent_gears(&grid, 2, 2);
        assert_eq!(set.len(), 3)
    }

    #[test]
    fn it_collects_all_numbers_around_a_gear() {
        let input = ".....\n.123.\n..*4.\n.2132\n.....";
        let grid = create_char_grid_from_string(&input);
        let mut gear_nums: HashMap<String, Vec<u16>> = HashMap::new();
        grid_collect_gear_adj_numbers(&grid, &mut gear_nums);

        for (gear, nums) in gear_nums {
            assert_eq!(gear, "2,2");
            assert_eq!(nums.len(), 3);
        }
    }


}