pub fn format_number(number: &str) -> String {
    let mut raw_number: Vec<char> = number.chars().filter(char::is_ascii_digit).collect();
    let mut groups: Vec<String> = Vec::with_capacity(raw_number.len());
    while !raw_number.is_empty() {
        match raw_number.len() {
            4 => {
                for _ in 0..2 {
                    let group: String = raw_number.drain(..2).collect();
                    groups.push(group);
                }
            }
            2 => {
                let group: String = raw_number.drain(..2).collect();
                groups.push(group);
            }
            _ => {
                let group: String = raw_number.drain(..3).collect();
                groups.push(group);
            }
        }
    }
    groups.join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = format_number("1-23-45-6");
        assert_eq!(result, "123-456");
    }

    #[test]
    fn test_2() {
        let result = format_number("1234");
        assert_eq!(result, "12-34");
    }

    #[test]
    fn test_3() {
        let result = format_number("12 345-6789");
        assert_eq!(result, "123-456-789");
    }

    #[test]
    fn test_4() {
        let result = format_number("123 4567");
        assert_eq!(result, "123-45-67");
    }

    #[test]
    fn test_5() {
        let result = format_number("123 456-78");
        assert_eq!(result, "123-456-78");
    }
}
