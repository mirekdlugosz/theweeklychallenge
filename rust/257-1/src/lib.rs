pub fn smaller(ints: &[usize]) -> Vec<usize> {
    ints.iter().map(|i| {
        ints.iter().filter(|j| i > j).count()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = smaller(&vec![5, 2, 1, 6]);
        assert_eq!(result, vec![2, 1, 0, 3]);
    }

    #[test]
    fn case2() {
        let result = smaller(&vec![1, 2, 0, 3]);
        assert_eq!(result, vec![1, 2, 0, 3]);
    }

    #[test]
    fn case3() {
        let result = smaller(&vec![0, 1]);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn case4() {
        let result = smaller(&vec![9, 4, 9, 2]);
        assert_eq!(result, vec![2, 1, 2, 0]);
    }
}
