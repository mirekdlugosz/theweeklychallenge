use std::collections::HashMap;

pub fn popular_word(paragraph: &str, banned: &[&str]) -> String {
    let mut counter: HashMap<&str, usize> = HashMap::with_capacity(paragraph.len());

    let p = paragraph
        .replace(['.', ',', '?', '!', ':', ';', '-', '+', '=', '_'], " ")
        .to_ascii_lowercase();

    for word in p.split_ascii_whitespace() {
        counter.entry(word).and_modify(|c| *c += 1).or_insert(1);
    }

    for word in banned {
        counter.remove_entry(word);
    }

    if let Some((key, _val)) = counter.iter().max_by_key(|(_key, val)| **val) {
        key.to_string()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = popular_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.",
            &["hit"],
        );
        assert_eq!(result, "ball");
    }

    #[test]
    fn test2() {
        let result = popular_word(
            "Apple? apple! Apple, pear, orange, pear, apple, orange.",
            &["apple", "pear"],
        );
        assert_eq!(result, "orange");
    }

    #[test]
    fn test3() {
        let result = popular_word("A. a, a! A. B. b. b.", &["b"]);
        assert_eq!(result, "a");
    }

    #[test]
    fn test4() {
        let result = popular_word("Ball.ball,ball:apple!apple.banana", &["ball"]);
        assert_eq!(result, "apple");
    }

    #[test]
    fn test5() {
        let result = popular_word(
            "The dog chased the cat, but the dog was faster than the cat.",
            &["the", "dog"],
        );
        assert_eq!(result, "cat");
    }
}
