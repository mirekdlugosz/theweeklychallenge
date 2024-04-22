use std::collections::{HashSet, HashMap};


fn uncommon_words_impl(line1: &str, line2: &str) -> Vec<String> {
    let mut line1_counter: HashMap<&str, usize> = HashMap::with_capacity(line1.split(' ').count());
    let mut line1_words: HashSet<&str> = HashSet::with_capacity(line1.split(' ').count());
    let line2_words: HashSet<&str> = line2.split(' ').collect();

    for word in line1.split(' ') {
        line1_counter
            .entry(word)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for (key, value) in line1_counter.iter() {
        if *value == 1 {
            line1_words.insert(key);
        }
    }

    line1_words
        .iter()
        .filter(|&word| !(&line2_words.contains(word)))
        .map(|&word| word.to_string())
        .collect()
}


pub fn uncommon_words(line1: &str, line2: &str) -> Vec<String> {
    let line1_uncommon_words = uncommon_words_impl(line1, line2);
    let line2_uncommon_words = uncommon_words_impl(line2, line1);

    let rv = [line1_uncommon_words, line2_uncommon_words].concat();
    match rv.len() {
        0 => vec!("".to_string()),
        _ => rv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let line1 = "Mango is sweet";
        let line2 = "Mango is sour";
        let output = vec!(
            "sweet".to_string(),
            "sour".to_string(),
        );
        assert_eq!(uncommon_words(line1, line2), output);
    }

    #[test]
    fn test2() {
        let line1 = "Mango Mango";
        let line2 = "Orange";
        let output = vec!(
            "Orange".to_string(),
        );
        assert_eq!(uncommon_words(line1, line2), output);
    }

    #[test]
    fn test3() {
        let line1 = "Mango is Mango";
        let line2 = "Orange is Orange";
        let output = vec!(
            "".to_string(),
        );
        assert_eq!(uncommon_words(line1, line2), output);
    }

    #[test]
    fn test4() {
        let line1 = "Mango Mango";
        let line2 = "Mango";
        let output = vec!(
            "".to_string(),
        );
        assert_eq!(uncommon_words(line1, line2), output);
    }
}
