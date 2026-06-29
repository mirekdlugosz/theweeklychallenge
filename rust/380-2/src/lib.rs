static LETTERS: [char; 26] = [
    'z', 'y', 'x', 'w', 'v', 'u', 't', 's', 'r', 'q', 'p', 'o', 'n', 'm', 'l', 'k', 'j', 'i', 'h',
    'g', 'f', 'e', 'd', 'c', 'b', 'a',
];

pub fn reverse_degree(str: &str) -> usize {
    str.chars()
        .enumerate()
        .map(|(pos, ch)| {
            LETTERS
                .iter()
                .position(|&n| n == ch)
                .map_or(0, |p| (p + 1) * (pos + 1))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = reverse_degree("z");
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = reverse_degree("a");
        assert_eq!(result, 26);
    }

    #[test]
    fn test3() {
        let result = reverse_degree("bbc");
        assert_eq!(result, 147);
    }

    #[test]
    fn test4() {
        let result = reverse_degree("racecar");
        assert_eq!(result, 560);
    }

    #[test]
    fn test5() {
        let result = reverse_degree("zyx");
        assert_eq!(result, 14);
    }
}
