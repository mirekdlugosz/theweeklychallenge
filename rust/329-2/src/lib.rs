use std::collections::HashSet;

fn is_nice_string(str: &str) -> bool {
    let mut seen: HashSet<char> = HashSet::with_capacity(str.len());

    for (idx, ch) in str.chars().enumerate() {
        let key = ch.to_ascii_lowercase();
        if seen.contains(&key) {
            continue;
        }

        let other_case = if ch.is_ascii_lowercase() {
            ch.to_ascii_uppercase()
        } else {
            ch.to_ascii_lowercase()
        };
        if let Some(_) = str.chars().skip(idx).find(|&c| c == other_case) {
            seen.insert(key);
            continue;
        }
        return false;
    }
    true
}

pub fn nice_string(str: &str) -> &str {
    let string_len = str.chars().count();
    let mut target_len = string_len;

    while target_len > 0 {
        for shift in 0..=(string_len - target_len) {
            if let Some(s) = str.get(shift..(shift + target_len)) {
                if is_nice_string(s) {
                    return s;
                }
            }
        }
        target_len -= 1;
    }
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = nice_string("YaaAho");
        assert_eq!(result, "aaA");
    }

    #[test]
    fn test2() {
        let result = nice_string("cC");
        assert_eq!(result, "cC");
    }

    #[test]
    fn test3() {
        let result = nice_string("A");
        assert_eq!(result, "");
    }
}
