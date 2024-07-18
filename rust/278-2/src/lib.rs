pub fn reverse_word(word: &str, char: char) -> String {
    let char_pos = word.find(|c: char| c == char);

    match char_pos {
        None => word.into(),
        Some(char_pos) => {
            let mut output = String::new();
            let to_sort = &word[0..=char_pos];
            let unsorted = &word[char_pos+1..];
            let mut to_sort_vec: Vec<char> = to_sort.chars().collect();
            to_sort_vec.sort_unstable();
            for c in to_sort_vec {
                output.push(c);
            }
            output.push_str(unsorted);
            output
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = reverse_word("challenge", 'e');
        assert_eq!(result, "acehllnge".to_string());
    }

    #[test]
    fn test2() {
        let result = reverse_word("programming", 'a');
        assert_eq!(result, "agoprrmming".to_string());
    }

    #[test]
    fn test3() {
        let result = reverse_word("champion", 'b');
        assert_eq!(result, "champion".to_string());
    }
}
