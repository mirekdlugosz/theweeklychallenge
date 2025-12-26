pub fn match_string(words: &[&str]) -> Vec<String> {
    let mut new_words: Vec<String> = Vec::with_capacity(words.len());
    for word in words {
        if new_words.contains(&word.to_string()) {
            continue;
        }

        if words
            .iter()
            .any(|new_word| new_word != word && new_word.contains(word))
        {
            new_words.push(word.to_string());
        }
    }
    new_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = match_string(&[
            "cat",
            "cats",
            "dog",
            "dogcat",
            "dogcat",
            "rat",
            "ratcatdogcat",
        ]);
        let expected = vec!["cat", "dog", "dogcat", "rat"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = match_string(&["hello", "hell", "world", "wor", "ellow", "elloworld"]);
        let expected = vec!["hell", "world", "wor", "ellow"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = match_string(&["a", "aa", "aaa", "aaaa"]);
        let expected = vec!["a", "aa", "aaa"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let result = match_string(&["flower", "flow", "flight", "fl", "fli", "ig", "ght"]);
        let expected = vec!["flow", "fl", "fli", "ig", "ght"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let result = match_string(&["car", "carpet", "carpenter", "pet", "enter", "pen", "pent"]);
        let expected = vec!["car", "pet", "enter", "pen", "pent"];
        assert_eq!(result, expected);
    }
}
