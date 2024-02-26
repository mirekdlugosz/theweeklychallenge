pub fn sum_of_values(ints: &[usize], k: u32) -> usize {
    ints.iter()
        .enumerate()
        .filter_map(|(i, v)| {
            match i.count_ones() == k {
                true => Some(v),
                false => None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        let input = vec![2, 5, 9, 11, 3];
        assert_eq!(sum_of_values(&input, 1), 17);
    }

    #[test]
    fn task2() {
        let input = vec![2, 5, 9, 11, 3];
        assert_eq!(sum_of_values(&input, 2), 11);
    }
    #[test]

    fn task3() {
        let input = vec![2, 5, 9, 11, 3];
        assert_eq!(sum_of_values(&input, 0), 2);
    }
}
