pub fn changed_keys(str: &str) -> usize {
    let mut keys_changed_no = 0;
    let mut chars = str.chars();
    let mut current_char = chars.next().unwrap().to_ascii_lowercase();

    for ch in chars {
        let new_char = ch.to_ascii_lowercase();
        if new_char == current_char {
            continue;
        }
        keys_changed_no += 1;
        current_char = new_char;
    }

    keys_changed_no
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = changed_keys("pPeERrLl");
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = changed_keys("rRr");
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let result = changed_keys("GoO");
        assert_eq!(result, 1);
    }
}
