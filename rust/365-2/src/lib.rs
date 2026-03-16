use std::collections::HashMap;

static SPECIAL_CHARS: [char; 4] = ['-', ',', '.', '!'];

fn is_valid(str: &str) -> bool {
    if str.chars().any(|c| c.is_ascii_digit()) {
        return false;
    }

    let str_len = str.chars().count();

    let mut special_chars: HashMap<char, usize> = HashMap::with_capacity(4);

    for c in str.chars() {
        if SPECIAL_CHARS.contains(&c) {
            special_chars.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    if !special_chars.is_empty() {
        if special_chars.values().any(|v| *v > 1) {
            return false;
        }

        for sch in SPECIAL_CHARS {
            if special_chars.contains_key(&sch)
                && let Some(sch_pos) = str.chars().position(|c| c == sch)
            {
                let is_hyphen = sch == '-';
                let is_last = (sch_pos + 1) == str_len;
                let is_first = sch_pos == 0;

                if (is_hyphen && (is_first || is_last)) || (!is_hyphen && !is_last) {
                    return false;
                }
            }
        }
    }

    true
}

pub fn valid_token_counter(str: &str) -> usize {
    str.split_whitespace().filter(|e| is_valid(e)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = valid_token_counter("cat and dog");
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = valid_token_counter("a-b c! d,e");
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let result = valid_token_counter("hello-world! this is fun");
        assert_eq!(result, 4);
    }

    #[test]
    fn test4() {
        let result = valid_token_counter("ab- cd-ef gh- ij!");
        assert_eq!(result, 2);
    }

    #[test]
    fn test5() {
        let result = valid_token_counter("wow! a-b-c nice.");
        assert_eq!(result, 2);
    }
}
