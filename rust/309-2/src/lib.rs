pub fn min_diff(ints: &[isize]) -> usize {
    ints
        .iter()
        .enumerate()
        .filter_map(|(idx, &n)| {
            ints
                .iter()
                .skip(idx + 1)
                .map(|x| x.abs_diff(n))
                .min()
        })
        .min()
        .unwrap_or(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = min_diff(&[1, 5, 8, 9]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = min_diff(&[9, 4, 1, 7]);
        assert_eq!(result, 2);
    }
}
