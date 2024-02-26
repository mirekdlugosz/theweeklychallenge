pub fn even_digits(ints: &[usize]) -> usize {
    ints.iter()
        .map(|i| format!("{i}"))
        .filter(|s| s.len() % 2 == 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![10, 1, 111, 24, 1000];
        assert_eq!(even_digits(&input), 3);
    }

    #[test]
    fn case2() {
        let input = vec![111, 1, 11111];
        assert_eq!(even_digits(&input), 0);
    }

    #[test]
    fn case3() {
        let input = vec![2, 8, 1024, 256];
        assert_eq!(even_digits(&input), 1);
    }
}
