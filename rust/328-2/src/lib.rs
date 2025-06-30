pub fn good_string(str: &str) -> String {
    let mut new_str: Vec<char> = str.chars().collect();
    let mut changed: bool;

    loop {
        changed = false;
        for char_pos in 1..=new_str.len() {
            let prev_pos = char_pos - 1;
            let prev_char = new_str.get(prev_pos).unwrap_or(&'\0');
            let current_char = new_str.get(char_pos).unwrap_or(&'0');
            if prev_char.eq_ignore_ascii_case(current_char)
                && ((prev_char.is_ascii_lowercase() && current_char.is_ascii_uppercase())
                    || (prev_char.is_ascii_uppercase() && current_char.is_ascii_lowercase()))
            {
                new_str.remove(char_pos);
                new_str.remove(prev_pos);
                changed = true;
                break;
            }
        }
        if !changed {
            break;
        }
    }
    new_str.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = good_string("WeEeekly");
        assert_eq!(result, "Weekly");
    }

    #[test]
    fn test2() {
        let result = good_string("abBAdD");
        assert_eq!(result, "");
    }

    #[test]
    fn test3() {
        let result = good_string("abc");
        assert_eq!(result, "abc");
    }
}
