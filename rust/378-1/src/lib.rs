use std::collections::HashSet;

pub fn second_largest_digit(str: &str) -> isize {
    let mut digits: Vec<_> = str
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    digits.sort_unstable();
    digits.reverse();
    digits
        .get(1)
        .map_or(-1, |d| isize::try_from(*d).unwrap_or(-1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = second_largest_digit("aaaaa77777");
        assert_eq!(result, -1);
    }

    #[test]
    fn test2() {
        let result = second_largest_digit("abcde");
        assert_eq!(result, -1);
    }

    #[test]
    fn test3() {
        let result = second_largest_digit("9zero8eight7seven9");
        assert_eq!(result, 8);
    }

    #[test]
    fn test4() {
        let result = second_largest_digit("xyz9876543210");
        assert_eq!(result, 8);
    }

    #[test]
    fn test5() {
        let result = second_largest_digit("4abc4def2ghi8jkl2");
        assert_eq!(result, 4);
    }
}
