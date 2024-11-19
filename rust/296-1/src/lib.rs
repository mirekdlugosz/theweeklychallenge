pub fn compress_string(string: &str) -> String {
    string.chars()
        .collect::<Vec<_>>()
        .chunk_by(|a, b| a == b)
        .map(|chunk| {
            if let Some(char) = chunk.first() {
                match chunk.len() {
                    1 => char.to_string(),
                    n => format!("{}{}", n, char),
                }
            } else {
                "".to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = compress_string("abbc");
        assert_eq!(result, "a2bc".to_string());
    }

    #[test]
    fn test2() {
        let result = compress_string("aaabccc");
        assert_eq!(result, "3ab3c".to_string());
    }

    #[test]
    fn test3() {
        let result = compress_string("abcc");
        assert_eq!(result, "ab2c".to_string());
    }

    #[test]
    fn test4() {
        let result = compress_string("aabaacc");
        assert_eq!(result, "2ab2a2c".to_string());
    }
}
