pub fn max_words(sentences: &[&str]) -> usize {
    sentences
        .iter()
        .map(|sentence| sentence.split(' ').count())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = max_words(&["Hello world", "This is a test", "Perl is great"]);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let result = max_words(&["Single"]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let result = max_words(&[
            "Short",
            "This sentence has seven words in total",
            "A B C",
            "Just four words here",
        ]);
        assert_eq!(result, 7);
    }

    #[test]
    fn test4() {
        let result = max_words(&["One", "Two parts", "Three part phrase", ""]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test5() {
        let result = max_words(&[
            "The quick brown fox jumps over the lazy dog",
            "A",
            "She sells seashells by the seashore",
            "To be or not to be that is the question",
        ]);
        assert_eq!(result, 10);
    }
}
