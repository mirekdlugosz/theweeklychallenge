pub fn abs_diff(ints: &[usize]) -> usize {
    let elem_sum: usize = ints.iter().sum();
    let digit_sum: usize = ints
        .iter()
        .flat_map(|num| -> Vec<usize> {
            let numbers = format!("{num}");
            numbers
                .chars()
                .filter_map(|d| d.to_digit(10))
                .map(|d| {
                    usize::try_from(d).unwrap()
                })
                .collect()
        })
        .sum();
    elem_sum.abs_diff(digit_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ints = vec![1,2,3,45];
        assert_eq!(abs_diff(&ints), 36);
    }

    #[test]
    fn test2() {
        let ints = vec![1,12,3];
        assert_eq!(abs_diff(&ints), 9);
    }

    #[test]
    fn test3() {
        let ints = vec![1,2,3,4];
        assert_eq!(abs_diff(&ints), 0);
    }

    #[test]
    fn test4() {
        let ints = vec![236, 416, 336, 350];
        assert_eq!(abs_diff(&ints), 1296);
    }
}
