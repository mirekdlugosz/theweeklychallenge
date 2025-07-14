pub fn clear_digits(str: &str) -> String {
    let mut new_string: Vec<char> = str.chars().collect();
    while let Some(num_idx) = new_string.iter().position(char::is_ascii_digit) {
        if let Some(prev_num_idx) = num_idx.checked_sub(1) {
            new_string.remove(num_idx);
            new_string.remove(prev_num_idx);
        } else {
            break;
        }
    }
    new_string.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = clear_digits("cab12");
        assert_eq!(result, "c");
    }

    #[test]
    fn test2() {
        let result = clear_digits("xy99");
        assert_eq!(result, "");
    }

    #[test]
    fn test3() {
        let result = clear_digits("pa1erl");
        assert_eq!(result, "perl");
    }
}
