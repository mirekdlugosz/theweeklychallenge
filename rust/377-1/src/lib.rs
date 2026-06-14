pub fn reverse_existence(str: &str) -> bool {
    let reversed: String = str.chars().rev().collect();
    str.chars().collect::<Vec<_>>().windows(2).any(|n| {
        let needle: String = n.iter().collect();
        reversed.contains(&needle)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = reverse_existence("abcba");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = reverse_existence("racecar");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = reverse_existence("abcd");
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = reverse_existence("banana");
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = reverse_existence("hello");
        assert_eq!(result, true);
    }
}
