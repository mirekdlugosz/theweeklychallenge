use std::collections::HashSet;

pub fn greatest_letter(str: &str) -> String {
    let mut lowercase: HashSet<char> = HashSet::with_capacity(str.chars().count());
    let mut uppercase: HashSet<char> = HashSet::with_capacity(str.chars().count());

    for char in str.chars() {
        if char.is_lowercase() {
            lowercase.insert(char.to_ascii_uppercase());
        } else {
            uppercase.insert(char);
        }
    }

    match lowercase.intersection(&uppercase).max_by_key(|ch| **ch as u8) {
        Some(ch) => ch.to_string(),
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(greatest_letter("PeRlwEeKLy"), "L".to_string());
    }

    #[test]
    fn test2() {
        assert_eq!(greatest_letter("ChaLlenge"), "L".to_string());
    }

    #[test]
    fn test3() {
        assert_eq!(greatest_letter("The"), "".to_string());
    }
}
