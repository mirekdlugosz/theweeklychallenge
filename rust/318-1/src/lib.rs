pub fn group_position(str: &str) -> Vec<String> {
    str.chars()
        .collect::<Vec<char>>()
        .chunk_by(|a, b| a == b)
        .filter(|chunk| chunk.len() >= 3)
        .map(|f| f.iter().collect())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = group_position("abccccd");
        assert_eq!(result, ["cccc"]);
    }

    #[test]
    fn test2() {
        let result = group_position("aaabcddddeefff");
        assert_eq!(result, ["aaa", "dddd", "fff"]);
    }

    #[test]
    fn test3() {
        let result = group_position("abcdd");
        let expected: Vec<String> = Vec::new();
        assert_eq!(result, expected);
    }
}
