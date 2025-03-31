pub fn find_third(sentence: &str, first: &str, second: &str) -> Vec<String> {
    sentence
        .trim_end_matches('.')
        .split(' ')
        .collect::<Vec<_>>()
        .windows(3)
        .filter_map(|window| {
            let matching_first = window.first().is_some_and(|w| first == *w);
            let matching_second = window.get(1).is_some_and(|w| second == *w);
            if matching_first && matching_second {
                let output = window.last().map(|&s| String::from(s));
                output
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = find_third(
            "Perl is a my favourite language but Python is my favourite too.",
            "my",
            "favourite",
        );
        assert_eq!(result, vec!["language", "too"]);
    }

    #[test]
    fn test2() {
        let result = find_third(
            "Barbie is a beautiful doll also also a beautiful princess.",
            "a",
            "beautiful",
        );
        assert_eq!(result, vec!["doll", "princess"]);
    }

    #[test]
    fn test3() {
        let result = find_third("we will we will rock you rock you.", "we", "will");
        assert_eq!(result, vec!["we", "rock"]);
    }
}
