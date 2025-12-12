pub fn power_string(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .chunk_by(|a, b| a == b)
        .map(<[char]>::len)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = power_string("textbook");
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = power_string("aaaaa");
        assert_eq!(result, 5);
    }

    #[test]
    fn test3() {
        let result = power_string("hoorayyy");
        assert_eq!(result, 3);
    }

    #[test]
    fn test4() {
        let result = power_string("x");
        assert_eq!(result, 1);
    }

    #[test]
    fn test5() {
        let result = power_string("aabcccddeeffffghijjk");
        assert_eq!(result, 4);
    }
}
