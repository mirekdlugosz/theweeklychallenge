pub fn highest_row(matrix: &[&[isize]]) -> isize {
    matrix.iter().map(|r| r.iter().sum()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = highest_row(&[&[4, 4, 4, 4], &[10, 0, 0, 0], &[2, 2, 2, 9]]);
        assert_eq!(result, 16);
    }

    #[test]
    fn test2() {
        let result = highest_row(&[&[1, 5], &[7, 3], &[3, 5]]);
        assert_eq!(result, 10);
    }

    #[test]
    fn test3() {
        let result = highest_row(&[&[1, 2, 3], &[3, 2, 1]]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test4() {
        let result = highest_row(&[&[10, 20, 30], &[5, 5, 5], &[0, 100, 0], &[25, 25, 25]]);
        assert_eq!(result, 100);
    }

    #[test]
    fn test5() {
        let result = highest_row(&[&[4, 4, 4, 4], &[10, 0, 0, 0], &[2, 2, 2, 9]]);
        assert_eq!(result, 16);
    }
}
