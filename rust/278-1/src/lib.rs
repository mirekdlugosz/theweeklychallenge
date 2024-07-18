use std::collections::BTreeMap;

pub fn sort_string(shuffle_string: &str) -> String {
    let mut sorted: BTreeMap<usize, &str> = BTreeMap::new();

    for word in shuffle_string.split(' ') {
        let pos = word.find(|c: char| c.is_numeric()).unwrap();
        let actual_word = &word[0..pos];
        let word_position: usize = word[pos..].parse().unwrap();
        sorted.insert(word_position, actual_word);
    }

    sorted.into_values()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = sort_string("and2 Raku3 cousins5 Perl1 are4");
        let expected = "Perl and Raku are cousins".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = sort_string("guest6 Python1 most4 the3 popular5 is2 language7");
        let expected = "Python is the most popular guest language".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = sort_string("Challenge3 The1 Weekly2");
        let expected = "The Weekly Challenge".to_string();
        assert_eq!(result, expected);
    }
}
