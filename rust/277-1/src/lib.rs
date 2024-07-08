use std::collections::{HashMap, HashSet};

fn single_words<'a>(words: &[&'a str]) -> HashSet<&'a str> {
    let mut words_counter: HashMap<&str, usize> = HashMap::with_capacity(words.len());

    for word in words {
        words_counter
            .entry(*word)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    words_counter.iter()
        .filter_map(|(key, value)| {
            match *value {
                1 => Some(*key),
                _ => None
            }
        })
        .collect()

}

pub fn count_common(words1: &[&str], words2: &[&str]) -> usize {
    let words1_set = single_words(words1);
    let words2_set = single_words(words2);

    words1_set.intersection(&words2_set).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words1 = ["Perl", "is", "my", "friend"];
        let words2 = ["Perl", "and", "Raku", "are", "friend"];
        assert_eq!(count_common(&words1, &words2), 2);
    }

    #[test]
    fn test2() {
        let words1 = ["Perl", "and", "Python", "are", "very", "similar"];
        let words2 = ["Python", "is", "top", "in", "guest", "languages"];
        assert_eq!(count_common(&words1, &words2), 1);
    }

    #[test]
    fn test3() {
        let words1 = ["Perl", "is", "imperative", "Lisp", "is", "functional"];
        let words2 = ["Crystal", "is", "similar", "to", "Ruby"];
        assert_eq!(count_common(&words1, &words2), 0);
    }
}
