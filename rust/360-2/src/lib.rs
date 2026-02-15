pub fn word_sorter(str: &str) -> String {
    let mut words: Vec<&str> = str.split_whitespace().collect();
    words.sort_unstable_by_key(|a| a.to_ascii_lowercase());
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = word_sorter("The quick brown fox");
        assert_eq!(result, "brown fox quick The");
    }

    #[test]
    fn test2() {
        let result = word_sorter("Hello    World!   How   are you?");
        assert_eq!(result, "are Hello How World! you?");
    }

    #[test]
    fn test3() {
        let result = word_sorter("Hello");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test4() {
        let result = word_sorter("Hello, World! How are you?");
        assert_eq!(result, "are Hello, How World! you?");
    }

    #[test]
    fn test5() {
        let result = word_sorter("I have 2 apples and 3 bananas!");
        assert_eq!(result, "2 3 and apples bananas! have I");
    }
}
