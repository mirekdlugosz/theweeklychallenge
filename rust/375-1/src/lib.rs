use std::collections::{HashMap, HashSet};

fn as_hashmap(array: &[&str]) -> HashSet<String> {
    let mut o = HashMap::with_capacity(array.len());
    for word in array {
        let w = word.to_string();
        o.entry(w).and_modify(|c| *c += 1).or_insert(1);
    }
    o.iter()
        .filter_map(|(key, val)| {
            if *val == 1 {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn single_common_word(array1: &[&str], array2: &[&str]) -> usize {
    let h1 = as_hashmap(array1);
    let h2 = as_hashmap(array2);

    h1.intersection(&h2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = single_common_word(
            &["apple", "banana", "cherry"],
            &["banana", "cherry", "date"],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = single_common_word(&["a", "ab", "abc"], &["a", "a", "ab", "abc"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let result = single_common_word(&["orange", "lemon"], &["grape", "melon"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = single_common_word(&["test", "test", "demo"], &["test", "demo", "demo"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test5() {
        let result = single_common_word(&["Hello", "world"], &["hello", "world"]);
        assert_eq!(result, 1);
    }
}
