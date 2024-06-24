use std::collections::HashSet;

pub fn count_fully_typed(sentence: &str, keys: &[char]) -> usize {
    let keys_set: HashSet<char> = keys.iter().flat_map(|ch| ch.to_lowercase()).collect();

    sentence
        .split(' ')
        .filter(|word| {
            let word_set: HashSet<char> = word
                .to_lowercase()
                .chars()
                .collect();

            keys_set.intersection(&word_set).count() == 0
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = count_fully_typed("Perl Weekly Challenge", &['l', 'a']);
        assert_eq!(result, 0);
    }

    #[test]
    fn test2() {
        let result = count_fully_typed("Perl and Raku", &['a']);
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let result = count_fully_typed("Well done Team PWC", &['l', 'o']);
        assert_eq!(result, 2);
    }

    #[test]
    fn test4() {
        let result = count_fully_typed("The joys of polyglottism", &['T']);
        assert_eq!(result, 2);
    }
}
