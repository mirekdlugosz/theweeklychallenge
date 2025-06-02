pub fn array_2d(ints: &[isize], r: usize, c: usize) -> Vec<Vec<isize>> {
    (0..r)
        .map(|ridx| ints.iter().skip(ridx * c).take(c).copied().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = array_2d(&[1, 2, 3, 4], 2, 2);
        assert_eq!(result, [[1, 2], [3, 4]]);
    }

    #[test]
    fn test2() {
        let result = array_2d(&[1, 2, 3], 1, 3);
        assert_eq!(result, [[1, 2, 3]]);
    }

    #[test]
    fn test3() {
        let result = array_2d(&[1, 2, 3, 4], 4, 1);
        assert_eq!(result, [[1], [2], [3], [4]]);
    }
}
