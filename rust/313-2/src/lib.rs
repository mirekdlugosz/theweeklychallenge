pub fn reverse_letters(str: &str) -> String {
    let capacity = str.len();
    let mut non_letters: Vec<_> = Vec::with_capacity(capacity);
    let mut letters: Vec<_> = Vec::with_capacity(capacity);

    for (idx, ch) in str.chars().enumerate() {
        if ch.is_alphabetic() {
            letters.push(ch);
        } else {
            non_letters.push((idx, ch));
        }
    }

    letters.reverse();

    for (idx, ch) in &non_letters {
        letters.insert(*idx, *ch);
    }

    letters.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = reverse_letters("p-er?l");
        assert_eq!(result, "l-re?p");
    }

    #[test]
    fn test2() {
        let result = reverse_letters("wee-k!L-y");
        assert_eq!(result, "yLk-e!e-w");
    }

    #[test]
    fn test3() {
        let result = reverse_letters("_c-!h_all-en!g_e");
        assert_eq!(result, "_e-!g_nel-la!h_c");
    }
}
