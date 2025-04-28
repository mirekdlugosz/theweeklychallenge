static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn word_count(list: &[&str]) -> usize {
    list.iter()
        .filter(|word| {
            let first_letter = word.chars().next().unwrap_or('\0');
            let last_letter = word.chars().next_back().unwrap_or('\0');
            VOWELS.contains(&first_letter) || VOWELS.contains(&last_letter)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = word_count(&["unicode", "xml", "raku", "perl"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = word_count(&["the", "weekly", "challenge"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let result = word_count(&["perl", "python", "postgres"]);
        assert_eq!(result, 0);
    }
}
