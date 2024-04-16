use itertools::Itertools;


pub fn are_words_completing(this: &str, another: &str) -> bool {
    let this_counts = this.chars().counts();
    let another_counts = another.chars().counts();

    for (letter, count) in this_counts.iter() {
        if let Some(another_count) = another_counts.get(letter) {
            if another_count >= count {
                continue;
            }
        }
        return false
    }
    true
}


pub fn completing_word(str: &str, strings: &[&str]) -> String {
    let transformed_str: String = str
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();
    
    strings
        .iter()
        .filter(|candidate| are_words_completing(&transformed_str, candidate))
        .min_by(|a, b| (a.chars().count()).cmp(&b.chars().count()))
        .copied()
        .unwrap_or("")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strings = vec!("accbbb", "abc", "abbc");
        assert_eq!(completing_word("aBc 11c", &strings), "accbbb".to_string());
    }

    #[test]
    fn test2() {
        let strings = vec!("abcm", "baacd", "abaadc");
        assert_eq!(completing_word("Da2 abc", &strings), "baacd".to_string());
    }

    #[test]
    fn test3() {
        let strings = vec!("jj", "bb", "bjb");
        assert_eq!(completing_word("JB 007", &strings), "bjb".to_string());
    }

    #[test]
    fn test4() {
        let strings = vec!("bbjjbb", "bbjb", "bjb");
        assert_eq!(completing_word("JB 007", &strings), "bjb".to_string());
    }

    #[test]
    fn test5() {
        let strings = vec!("jj", "bb", "bjb");
        assert_eq!(completing_word("AB 007", &strings), "".to_string());
    }
}
