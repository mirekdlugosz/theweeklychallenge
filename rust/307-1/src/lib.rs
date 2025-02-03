pub fn check_order(ints: &[isize]) -> Vec<usize> {
    let mut sorted: Vec<isize> = ints.into();
    sorted.sort_unstable();

    ints.iter()
        .zip(sorted)
        .enumerate()
        .filter_map(
            |(idx, (this, other))| {
                if other.eq(this) {
                    None
                } else {
                    Some(idx)
                }
            },
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = check_order(&[5, 2, 4, 3, 1]);
        let expected = vec![0, 2, 3, 4];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = check_order(&[1, 2, 1, 1, 3]);
        let expected = vec![1, 3];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = check_order(&[3, 1, 3, 2, 3]);
        let expected = vec![0, 1, 3];
        assert_eq!(result, expected);
    }
}
