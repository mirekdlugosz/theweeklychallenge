// Conceptually it's pretty simple: split string into digits and non-digits.
// If one group has at least 2 elements more than the other, bail out - string can't be balanced.
// You generally want to start with digits and put non-digit as second element. However, if there
// are more non-digits than digits, you have to start with non-digit.
pub fn balance_string(str: &str) -> String {
    let (mut digits, mut chars): (Vec<_>, Vec<_>) = str.chars().partition(char::is_ascii_digit);

    // This is potentially wasteful, because we already know if string can't be balanced.
    // We could exit early, but that will result in less straightforward code. Hope compiler
    // does the right thing.
    digits.sort_unstable();
    chars.sort_unstable();

    match digits.len().abs_diff(chars.len()) {
        0 => digits
            .iter()
            .zip(chars.iter())
            .map(|(d, c)| format!("{d}{c}"))
            .collect(),
        1 => {
            let (first, second) = if digits.len() > chars.len() {
                (digits, chars)
            } else {
                (chars, digits)
            };
            let mut secondi = second.iter();
            let mut output = String::with_capacity(str.chars().count());
            for a in first.iter() {
                output.push(*a);
                if let Some(b) = secondi.next() {
                    output.push(*b);
                }
            }
            output
        }
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = balance_string("a0b1c2");
        assert_eq!(result, "0a1b2c");
    }

    #[test]
    fn test2() {
        let result = balance_string("abc12");
        assert_eq!(result, "a1b2c");
    }

    #[test]
    fn test3() {
        let result = balance_string("0a2b1c3");
        assert_eq!(result, "0a1b2c3");
    }

    #[test]
    fn test4() {
        let result = balance_string("1a23");
        assert_eq!(result, "");
    }

    #[test]
    fn test5() {
        let result = balance_string("ab123");
        assert_eq!(result, "1a2b3");
    }
}
