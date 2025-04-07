pub fn circular(list: &[&str]) -> bool {
    list.windows(2).all(|window| {
        let first_word = window
            .first()
            .map_or('\0', |w| w.chars().next_back().unwrap_or('\0'));
        let last_word = window
            .last()
            .map_or('1', |w| w.chars().next().unwrap_or('1'));
        first_word == last_word
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = circular(&["perl", "loves", "scala"]);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = circular(&["love", "the", "programming"]);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = circular(&["java", "awk", "kotlin", "node.js"]);
        assert_eq!(result, true);
    }
}
