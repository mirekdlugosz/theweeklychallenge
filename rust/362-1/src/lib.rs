use std::iter;

pub fn echo_chamber(input: &str) -> String {
    input
        .chars()
        .enumerate()
        .map(|(idx, ch)| iter::repeat_n(ch, idx.saturating_add(1)).collect::<String>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = echo_chamber("abca");
        assert_eq!(result, "abbcccaaaa");
    }

    #[test]
    fn test2() {
        let result = echo_chamber("xyz");
        assert_eq!(result, "xyyzzz");
    }

    #[test]
    fn test3() {
        let result = echo_chamber("code");
        assert_eq!(result, "coodddeeee");
    }

    #[test]
    fn test4() {
        let result = echo_chamber("hello");
        assert_eq!(result, "heelllllllooooo");
    }

    #[test]
    fn test5() {
        let result = echo_chamber("a");
        assert_eq!(result, "a");
    }
}
