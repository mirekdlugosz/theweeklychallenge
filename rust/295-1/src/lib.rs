pub fn word_break(istr: &str, words: &[&str]) -> bool {
    let mut new_str: String = istr.to_string();

    for word in words.iter() {
        let new_word = format!(" {} ", word);
        if new_str.contains(word) {
            new_str = new_str.replacen(word, &new_word, 1);
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = word_break("weeklychallenge", &["challenge", "weekly"]);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = word_break("perlrakuperl", &["raku", "perl"]);
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = word_break("sonsanddaughters", &["sons", "sand", "daughters"]);
        assert_eq!(result, false);
    }
}
