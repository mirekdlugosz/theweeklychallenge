pub fn acronyms(array: &[&str], word: &str) -> bool {
    let acronym: String = array.iter().filter_map(|w| w.chars().next()).collect();
    acronym == word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = acronyms(&["Perl", "Weekly", "Challenge"], "PWC");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = acronyms(&["Bob", "Charlie", "Joe"], "BCJ");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = acronyms(&["Morning", "Good"], "MM");
        assert_eq!(result, false);
    }
}
