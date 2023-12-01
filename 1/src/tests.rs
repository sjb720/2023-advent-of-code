#[cfg(test)]
mod tests {
    use crate::{is_number, get_first_number, get_last_number, calculate_code, split_by_new_line};

    #[test]
    fn byte_is_number() {
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
    fn byte_is_not_number() {
        assert!(!is_number(b'a'));
        assert!(!is_number(b' '));
        assert!(!is_number(b'q'));
    }

    #[test]
    fn get_first_number_from_string() {
        assert_eq!(get_first_number("asdf1234fdsa"), 1);
        assert_eq!(get_first_number("6sdf1234fdsa"), 6);
        assert_eq!(get_first_number("no number here!"), 0);
    }

    #[test]
    fn get_last_number_from_string() {
        assert_eq!(get_last_number("asdf1234fdsa"), 4);
        assert_eq!(get_last_number("6sdfasdfdsa"), 6);
        assert_eq!(get_last_number("no number here!"), 0);
    }


    #[test]
    fn caluclate_code_from_string() {
        assert_eq!(calculate_code("asdf4asdf1asfaf"), 41);
        assert_eq!(calculate_code("123541728037"), 17);
        assert_eq!(calculate_code("no number here!"), 0);
        assert_eq!(calculate_code("asdfasdfa9asdfasdf"), 99);
    }

    #[test]
    fn split_string_by_new_line() {
        let str = String::from("asdf\nasdfasdf\nasdfa\nsadfasdf");
        assert_eq!(split_by_new_line(&str).len(), 4);

        let str = String::from("");
        assert_eq!(split_by_new_line(&str).len(), 1);

        let str = String::from("asdfasdf\nasdfaf\n");
        assert_eq!(split_by_new_line(&str).len(), 3);
    }
}