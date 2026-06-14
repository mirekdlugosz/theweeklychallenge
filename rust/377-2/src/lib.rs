fn is_prefix_suffix(word1: &str, word2: &str) -> bool {
    let ltr = word1.starts_with(word2) && word1.ends_with(word2);
    let rtl = word2.starts_with(word1) && word2.ends_with(word1);
    ltr || rtl
}

pub fn prefix_suffix(array: &[&str]) -> usize {
    let mut matching = 0;
    for (idx, word1) in array.iter().enumerate() {
        for word2 in array.iter().skip(idx.saturating_add(1)) {
            if is_prefix_suffix(word1, word2) {
                matching += 1;
            }
        }
    }
    matching
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = prefix_suffix(&["a", "aba", "ababa", "aa"]);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let result = prefix_suffix(&["pa", "papa", "ma", "mama"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let result = prefix_suffix(&["abao", "ab"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = prefix_suffix(&["abab", "abab"]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test5() {
        let result = prefix_suffix(&["ab", "abab", "ababab"]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test6() {
        let result = prefix_suffix(&["abc", "def", "ghij"]);
        assert_eq!(result, 0);
    }
}
