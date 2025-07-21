pub fn last_word(str: &str) -> usize {
    str.split_whitespace()
        .last()
        .map_or(0, |word| word.chars().count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = last_word("The Weekly Challenge");
        assert_eq!(result, 9);
    }

    #[test]
    fn test2() {
        let result = last_word("   Hello   World    ");
        assert_eq!(result, 5);
    }

    #[test]
    fn test3() {
        let result = last_word("Let's begin the fun");
        assert_eq!(result, 3);
    }
}
