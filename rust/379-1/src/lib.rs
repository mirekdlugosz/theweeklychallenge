pub fn reverse_string(str: &str) -> String {
    let mut out: String = String::default();
    let tmp: Vec<char> = str.chars().collect();
    let mut size = tmp.len();
    loop {
        size = size.saturating_sub(1);
        if let Some(ch) = tmp.get(size) {
            out.push(*ch);
        }
        if size == 0 {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = reverse_string("");
        assert_eq!(result, "");
    }

    #[test]
    fn test2() {
        let result = reverse_string("reverse the given string");
        assert_eq!(result, "gnirts nevig eht esrever");
    }

    #[test]
    fn test3() {
        let result = reverse_string("Perl is Awesome");
        assert_eq!(result, "emosewA si lreP");
    }

    #[test]
    fn test4() {
        let result = reverse_string("v1.0.0-Beta!");
        assert_eq!(result, "!ateB-0.0.1v");
    }

    #[test]
    fn test5() {
        let result = reverse_string("racecar");
        assert_eq!(result, "racecar");
    }
}
