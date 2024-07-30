use std::collections::HashSet;

pub fn appears_twice(input: &str) -> char {
    let mut seen: HashSet<char> = HashSet::with_capacity(input.len());

    for ch in input.chars() {
        if seen.contains(&ch) {
            return ch;
        } else {
            seen.insert(ch);
        }
    }

    // this should be unreachable and exercise never says what should happen
    // if there's no character that appears twice in input
    '\0'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = appears_twice("acbddbca");
        assert_eq!(result, 'd');
    }

    #[test]
    fn test2() {
        let result = appears_twice("abccd");
        assert_eq!(result, 'c');
    }

    #[test]
    fn test3() {
        let result = appears_twice("abcdabbb");
        assert_eq!(result, 'a');
    }
}
