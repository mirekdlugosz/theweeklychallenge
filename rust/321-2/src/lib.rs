// we assume backspace at the beginning of string is noop (could be invalid string)
// this assumption prevents us from employing heuristic check - if string lengths
// after removing doubled number of '#' are different, then we know for sure strings
// are different
fn parse_backspace(str: &str) -> String {
    let mut s: Vec<char> = str.chars().collect();
    while let Some(backspace) = s.iter().position(|&c| c == '#') {
        s.remove(backspace);
        if let Some(prev) = backspace.checked_sub(1) {
            s.remove(prev);
        }
    }
    s.into_iter().collect()
}

pub fn backspace_compare(str1: &str, str2: &str) -> bool {
    let parsed_str1 = parse_backspace(str1);
    let parsed_str2 = parse_backspace(str2);
    parsed_str1 == parsed_str2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = backspace_compare("ab#c", "ad#c");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = backspace_compare("ab##", "a#b#");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = backspace_compare("a#b", "c");
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = backspace_compare("ac#b", "c");
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let result = backspace_compare("a#c#b##", "c");
        assert_eq!(result, false);
    }

    #[test]
    fn test6() {
        let result = backspace_compare("a##cb#", "c");
        assert_eq!(result, true);
    }
}
