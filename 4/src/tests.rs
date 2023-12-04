#[cfg(test)]
mod tests {
    use crate::{winning_numbers_string_to_hashset, numbers_to_vec, ticket_string_to_object, get_score};


    #[test]
    fn it_converts_a_winning_number_string_into_a_hash_set () {
        let set = winning_numbers_string_to_hashset(" 82 15 37 75 85 51 99 18 17  2 ");
        assert_eq!(set.len(), 10);
        assert_eq!(set.get(&2), Some(&2));
        assert_eq!(set.get(&12), None);
        assert_eq!(set.get(&82), Some(&82));
    }

    #[test]
    fn it_converts_a_number_string_into_a_vec () {
        let nums = numbers_to_vec("  8 17 54 14 18 99  4 85 51 49 91 15 82 24 75 25 69 61 52 58 37 87  2 22 28");
        assert_eq!(nums.len(), 25);
        assert_eq!(nums[0], 8);
        assert_eq!(nums[24], 28);
    }

    #[test]
    fn it_converts_a_ticket_string_to_an_objects () {
        let ticket = ticket_string_to_object("Card  12: 45 74 54  7  1 42 76  5 46 63 | 47 42  9 62 76 74 98 63 44  6 72  2 24  5 70 18 88  7 33 89 46  1 23 58 95");

        assert_eq!(ticket.numbers.len(),25);
        assert_eq!(ticket.winning_numbers.len(), 10);

        assert_eq!(ticket.numbers[0], 47);
        assert_eq!(ticket.numbers[24], 95);

        assert_eq!(ticket.winning_numbers.get(&1), Some(&1));
    }

    #[test]
    fn it_converts_a_ticket_to_object_and_gets_the_score () {
        // 8 winning numbers!!
        let ticket = ticket_string_to_object("Card  12: 45 74 54  7  1 42 76  5 46 63 | 47 42  9 62 76 74 98 63 44  6 72  2 24  5 70 18 88  7 33 89 46  1 23 58 95");

        assert_eq!(get_score(&ticket), 128)
    }


}