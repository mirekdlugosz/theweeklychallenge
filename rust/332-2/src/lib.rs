use std::collections::HashMap;

pub fn odd_letters(str: &str) -> bool {
    let mut letters: HashMap<char, usize> = HashMap::with_capacity(str.len());
    for ch in str.chars() {
        letters
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    letters.values().all(|count| count.rem_euclid(2) == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = odd_letters("weekly");
        assert_eq!(result, false);
    }

    #[test]
    fn test2() {
        let result = odd_letters("perl");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = odd_letters("challenge");
        assert_eq!(result, false);
    }
}
