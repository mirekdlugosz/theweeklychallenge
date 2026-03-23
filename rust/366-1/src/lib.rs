pub fn count_prefixes(array: &[&str], str: &str) -> usize {
    array.iter().filter(|&elem| str.starts_with(elem)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = count_prefixes(&["a", "ap", "app", "apple", "banana"], "apple");
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let result = count_prefixes(&["cat", "dog", "fish"], "bird");
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let result = count_prefixes(&["hello", "he", "hell", "heaven", "he"], "hello");
        assert_eq!(result, 4);
    }

    #[test]
    fn test4() {
        let result = count_prefixes(&["", "code", "coding", "cod"], "coding");
        assert_eq!(result, 3);
    }

    #[test]
    fn test5() {
        let result = count_prefixes(
            &["p", "pr", "pro", "prog", "progr", "progra", "program"],
            "program",
        );
        assert_eq!(result, 7);
    }
}
