pub fn thousand_separator(int: usize) -> String {
    int.to_string()
        .chars()
        .collect::<Vec<char>>()
        .rchunks(3)
        .map(|ch| ch.iter().copied().collect::<String>())
        .rev()
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = thousand_separator(123);
        assert_eq!(result, "123");
    }

    #[test]
    fn test2() {
        let result = thousand_separator(1234);
        assert_eq!(result, "1,234");
    }

    #[test]
    fn test3() {
        let result = thousand_separator(1000000);
        assert_eq!(result, "1,000,000");
    }

    #[test]
    fn test4() {
        let result = thousand_separator(1);
        assert_eq!(result, "1");
    }

    #[test]
    fn test5() {
        let result = thousand_separator(12345);
        assert_eq!(result, "12,345");
    }
}
