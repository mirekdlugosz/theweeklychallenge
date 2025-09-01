pub fn smaller_than_current(num1: &[isize]) -> Vec<usize> {
    num1.iter()
        .enumerate()
        .map(|(idx, reference)| {
            num1.iter()
                .enumerate()
                .filter(|(jdx, val)| idx != *jdx && reference >= val)
                .count()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = smaller_than_current(&[6, 5, 4, 8]);
        assert_eq!(result, [2, 1, 0, 3]);
    }

    #[test]
    fn test2() {
        let result = smaller_than_current(&[7, 7, 7, 7]);
        assert_eq!(result, [3, 3, 3, 3]);
    }

    #[test]
    fn test3() {
        let result = smaller_than_current(&[5, 4, 3, 2, 1]);
        assert_eq!(result, [4, 3, 2, 1, 0]);
    }

    #[test]
    fn test4() {
        let result = smaller_than_current(&[-1, 0, 3, -2, 1]);
        assert_eq!(result, [1, 2, 4, 0, 3]);
    }

    #[test]
    fn test5() {
        let result = smaller_than_current(&[0, 1, 1, 2, 0]);
        assert_eq!(result, [1, 3, 3, 4, 1]);
    }
}
