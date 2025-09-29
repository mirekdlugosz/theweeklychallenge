use std::collections::HashSet;

pub fn broken_keyboard(str: &str, keys: &HashSet<char>) -> usize {
    str.to_lowercase()
        .split_whitespace()
        .filter_map(|word| {
            let hash = HashSet::from_iter(word.chars());
            if hash.intersection(keys).count() == 0 {
                Some(1)
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = broken_keyboard("Hello World", &HashSet::from(['d']));
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = broken_keyboard("apple banana cherry", &HashSet::from(['a', 'e']));
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let result = broken_keyboard("Coding is fun", &HashSet::from([]));
        assert_eq!(result, 3);
    }

    #[test]
    fn test4() {
        let result = broken_keyboard("The Weekly Challenge", &HashSet::from(['a', 'b']));
        assert_eq!(result, 2);
    }

    #[test]
    fn test5() {
        let result = broken_keyboard("Perl and Python", &HashSet::from(['p']));
        assert_eq!(result, 1);
    }
}
