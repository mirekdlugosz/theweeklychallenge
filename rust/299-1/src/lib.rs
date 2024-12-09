pub fn replace_words(words: &[&str], sentence: &str) -> String {
    sentence
        .split(' ')
        .map(|word| {
            let found_word = words
                .iter()
                .find(|&substitute| word.starts_with(substitute));
            match found_word {
                Some(substitute) => substitute,
                None => word,
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = replace_words(
            &["cat", "bat", "rat"],
            "the cattle was rattle by the battery"
        );
        assert_eq!(result, "the cat was rat by the bat".to_string());
    }

    #[test]
    fn test2() {
        let result = replace_words(
            &["a", "b", "c"],
            "aab aac and cac bab"
        );
        assert_eq!(result, "a a a c b".to_string());
    }

    #[test]
    fn test3() {
        let result = replace_words(
            &["man", "bike"],
            "the manager was hit by a biker"
        );
        assert_eq!(result, "the man was hit by a bike".to_string());
    }
}
