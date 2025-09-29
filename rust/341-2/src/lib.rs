pub fn reverse_prefix(str: &str, char: char) -> String {
    let mut output = String::with_capacity(str.len());
    if let Some(idx) = str.chars().position(|c| c == char) {
        let mut rev = Vec::with_capacity(str.len());
        for c in str.chars().take(idx + 1) {
            rev.push(c);
        }
        rev.reverse();
        for c in rev {
            output.push(c);
        }
        for c in str.chars().skip(idx + 1) {
            output.push(c);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = reverse_prefix("programming", 'g');
        assert_eq!(result, "gorpramming");
    }

    #[test]
    fn test2() {
        let result = reverse_prefix("hello", 'h');
        assert_eq!(result, "hello");
    }

    #[test]
    fn test3() {
        let result = reverse_prefix("abcdefghij", 'h');
        assert_eq!(result, "hgfedcbaij");
    }

    #[test]
    fn test4() {
        let result = reverse_prefix("reverse", 's');
        assert_eq!(result, "srevere");
    }

    #[test]
    fn test5() {
        let result = reverse_prefix("perl", 'r');
        assert_eq!(result, "repl");
    }
}
