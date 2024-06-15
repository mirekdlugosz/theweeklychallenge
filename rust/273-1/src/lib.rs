pub fn char_percentage(str: &str, char: char) -> usize {
    let total = str.chars().count();
    let chars = str
        .chars()
        .filter(|x| *x == char)
        .count();

    let result = ((chars as f32 / total as f32) * 100.0).round();
    result as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = char_percentage("perl", 'e');
        assert_eq!(result, 25);
    }

    #[test]
    fn test2() {
        let result = char_percentage("java", 'a');
        assert_eq!(result, 50);
    }

    #[test]
    fn test3() {
        let result = char_percentage("python", 'm');
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = char_percentage("ada", 'a');
        assert_eq!(result, 67);
    }

    #[test]
    fn test5() {
        let result = char_percentage("ballerina", 'l');
        assert_eq!(result, 22);
    }

    #[test]
    fn test6() {
        let result = char_percentage("analitik", 'k');
        assert_eq!(result, 13);
    }
}
