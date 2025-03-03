pub fn upper_lower(str: &str) -> String {
    str.chars()
        .map(|ch| {
            if ch.is_ascii_lowercase() {
                ch.to_ascii_uppercase()
            } else {
                ch.to_ascii_lowercase()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = upper_lower("pERl");
        assert_eq!(result, "PerL".to_string());
    }

    #[test]
    fn test2() {
        let result = upper_lower("rakU");
        assert_eq!(result, "RAKu".to_string());
    }

    #[test]
    fn test3() {
        let result = upper_lower("PyThOn");
        assert_eq!(result, "pYtHoN".to_string());
    }
}
