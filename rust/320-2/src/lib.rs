pub fn sum_difference(ints: &[usize]) -> usize {
    let normal_sum: usize = ints.iter().sum();
    let digit_sum: usize = ints
        .iter()
        .map(usize::to_string)
        .collect::<String>()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap_or(0) as usize)
        .sum();
    normal_sum.abs_diff(digit_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = sum_difference(&[1, 23, 4, 5]);
        assert_eq!(result, 18);
    }

    #[test]
    fn test2() {
        let result = sum_difference(&[1, 2, 3, 4, 5]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let result = sum_difference(&[1, 2, 34]);
        assert_eq!(result, 27);
    }
}
