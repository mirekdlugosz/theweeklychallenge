pub fn zero_friend(nums: &[isize]) -> usize {
    let zero: isize = 0;
    nums.iter()
        .map(|&i| zero.abs_diff(i))
        .min()
        .unwrap_or(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = zero_friend(&[4, 2, -1, 3, -2]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = zero_friend(&[-5, 5, -3, 3, -1, 1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let result = zero_friend(&[7, -3, 0, 2, -8]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = zero_friend(&[-2, -5, -1, -8]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test5() {
        let result = zero_friend(&[-2, 2, -4, 4, -1, 1]);
        assert_eq!(result, 1);
    }
}
