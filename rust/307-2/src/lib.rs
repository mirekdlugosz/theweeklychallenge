use std::collections::HashMap;

fn as_hashmap(word: &str) -> HashMap<char, usize> {
    let mut hash = HashMap::new();
    for c in word.chars() {
        hash.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    hash
}

pub fn find_anagrams(words: &[&str]) -> usize {
    let mut words_hashes: Vec<_> = words.iter().map(|w| as_hashmap(w)).collect();

    'lop: loop {
        for (idx, window) in words_hashes.windows(2).enumerate() {
            let this = window.first().unwrap();
            let another = window.last().unwrap();
            if this.eq(another) {
                words_hashes.remove(idx);
                continue 'lop;
            }
        }
        break;
    }
    words_hashes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words = ["acca", "dog", "god", "perl", "repl"];
        let result = find_anagrams(&words);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let words = ["abba", "baba", "aabb", "ab", "ab"];
        let result = find_anagrams(&words);
        assert_eq!(result, 2);
    }
}
