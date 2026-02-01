pub fn max_str_value(strings: &[&str]) -> usize {
    strings
        .iter()
        .map(|s| usize::from_str_radix(s, 10).unwrap_or_else(|_| s.chars().count()))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = max_str_value(&["123", "45", "6"]);
        assert_eq!(result, 123);
    }

    #[test]
    fn test2() {
        let result = max_str_value(&["abc", "de", "fghi"]);
        assert_eq!(result, 4);
    }

    #[test]
    fn test3() {
        let result = max_str_value(&["0012", "99", "a1b2c"]);
        assert_eq!(result, 99);
    }

    #[test]
    fn test4() {
        let result = max_str_value(&["x", "10", "xyz", "007"]);
        assert_eq!(result, 10);
    }

    #[test]
    fn test5() {
        let result = max_str_value(&["hello123", "2026", "perl"]);
        assert_eq!(result, 2026);
    }
}
