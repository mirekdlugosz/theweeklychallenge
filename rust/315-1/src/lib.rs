pub fn find_words(list: &[&str], char: char) -> Vec<usize> {
    list.iter()
        .enumerate()
        .filter_map(
            |(idx, word)| {
                if word.contains(char) { Some(idx) } else { None }
            },
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = find_words(&["the", "weekly", "challenge"], 'e');
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test2() {
        let result = find_words(&["perl", "raku", "python"], 'p');
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn test3() {
        let result = find_words(&["abc", "def", "bbb", "bcd"], 'b');
        assert_eq!(result, vec![0, 2, 3]);
    }
}
