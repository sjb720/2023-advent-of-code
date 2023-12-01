#[cfg(test)]
mod tests {
    use crate::{is_number, get_first_number, get_last_number, calculate_code, split_by_new_line, get_number_from_start_of_string};

    #[test]
    fn it_confirms_the_byte_is_a_number() {
        assert!(is_number(b'0'));
        assert!(is_number(b'1'));
        assert!(is_number(b'2'));
        assert!(is_number(b'3'));
        assert!(is_number(b'4'));
        assert!(is_number(b'5'));
        assert!(is_number(b'6'));
        assert!(is_number(b'7'));
        assert!(is_number(b'8'));
        assert!(is_number(b'9'));
    }

    #[test]
    fn it_confirms_the_byte_is_not_a_number() {
        assert!(!is_number(b'a'));
        assert!(!is_number(b' '));
        assert!(!is_number(b'q'));
    }

    #[test]
    fn it_finds_the_first_number_in_the_string() {
        assert_eq!(get_first_number("asdf1234fdsa"), 1);
        assert_eq!(get_first_number("6sdf1234fdsa"), 6);
        assert_eq!(get_first_number("no number here!"), 0);

        assert_eq!(get_first_number("the first number is one, not two!"), 1);
        assert_eq!(get_first_number("one plus 1 is 2!"), 1);
        assert_eq!(get_first_number("six"), 6);
    }

    #[test]
    fn it_finds_the_last_number_in_the_string() {
        assert_eq!(get_last_number("asdf1234fdsa"), 4);
        assert_eq!(get_last_number("6sdfasdfdsa"), 6);
        assert_eq!(get_last_number("no number here!"), 0);

        assert_eq!(get_last_number("the first number is one, not two!"), 2);
        assert_eq!(get_last_number("one plus 1 is 2!"), 2);
        assert_eq!(get_last_number("six"), 6);

    }


    #[test]
    fn it_calculates_a_number_from_a_code_string() {
        assert_eq!(calculate_code("asdf4asdf1asfaf"), 41);
        assert_eq!(calculate_code("123541728037"), 17);
        assert_eq!(calculate_code("no number here!"), 0);
        assert_eq!(calculate_code("asdfasdfa9asdfasdf"), 99);

        assert_eq!(calculate_code("fiveandone"), 51);
        assert_eq!(calculate_code("eightthen1"), 81);
        assert_eq!(calculate_code("asdfhawoeufawevcusixfollowedby9four39ninafd"), 69);
        assert_eq!(calculate_code("nine"), 99);
    }

    #[test]
    fn it_splits_the_string_into_a_vec_by_new_line() {
        let str = String::from("asdf\nasdfasdf\nasdfa\nsadfasdf");
        assert_eq!(split_by_new_line(&str).len(), 4);

        let str = String::from("");
        assert_eq!(split_by_new_line(&str).len(), 1);

        let str = String::from("asdfasdf\nasdfaf\n");
        assert_eq!(split_by_new_line(&str).len(), 3);
    }

    #[test]
    fn it_finds_a_number_at_the_start_of_the_string() {
        assert_eq!(get_number_from_start_of_string("oneasdfe"), Ok(1));
        assert_eq!(get_number_from_start_of_string("two"), Ok(2));
        assert_eq!(get_number_from_start_of_string("three232"), Ok(3));
        assert_eq!(get_number_from_start_of_string("fourq23423asdfe"), Ok(4));
        assert_eq!(get_number_from_start_of_string("fiveeeeee"), Ok(5));
        assert_eq!(get_number_from_start_of_string("six"), Ok(6));
        assert_eq!(get_number_from_start_of_string("sevenasdf"), Ok(7));
        assert_eq!(get_number_from_start_of_string("eightasdf333"), Ok(8));
        assert_eq!(get_number_from_start_of_string("nine"), Ok(9));
    }

    #[test]
    fn it_does_not_find_a_number_at_the_start_of_the_string() {
        
        match get_number_from_start_of_string("oooooneasdfe") {
            Err(_) => {},
            Ok(_) => {
                panic!("Failed!");
            }
        }

        match get_number_from_start_of_string("9eight") {
            Err(_) => {},
            Ok(_) => {
                panic!("Failed!");
            }
        }

        match get_number_from_start_of_string("asdfasfeeightasdfasdf") {
            Err(_) => {},
            Ok(_) => {
                panic!("Failed!");
            }
        }

        match get_number_from_start_of_string("") {
            Err(_) => {},
            Ok(_) => {
                panic!("Failed!");
            }
        }

    }
}