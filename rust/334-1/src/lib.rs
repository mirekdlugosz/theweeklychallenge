pub fn range_sum(ints: &[isize], x: usize, y: usize) -> isize {
    ints.get(x..=y).map_or(0, |r| r.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = range_sum(&[-2, 0, 3, -5, 2, -1], 0, 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = range_sum(&[1, -2, 3, -4, 5], 1, 3);
        assert_eq!(result, -3);
    }

    #[test]
    fn test3() {
        let result = range_sum(&[1, 0, 2, -1, 3], 3, 4);
        assert_eq!(result, 2);
    }

    #[test]
    fn test4() {
        let result = range_sum(&[-5, 4, -3, 2, -1, 0], 0, 3);
        assert_eq!(result, -2);
    }

    #[test]
    fn test5() {
        let result = range_sum(&[-1, 0, 2, -3, -2, 1], 0, 2);
        assert_eq!(result, 1);
    }
}
