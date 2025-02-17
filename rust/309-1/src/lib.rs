pub fn min_gap(ints: &[isize]) -> isize {
    ints
        .windows(2)
        .map(|window| {
            let lower = window.first().unwrap_or(&isize::MIN);
            let upper = window.last().unwrap_or(&isize::MAX);
            (*upper, lower.abs_diff(*upper))
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .map_or(0, |x| x.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = min_gap(&[2, 8, 10, 11, 15]);
        assert_eq!(result, 11);
    }

    #[test]
    fn test2() {
        let result = min_gap(&[1, 5, 6, 7, 14]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test3() {
        let result = min_gap(&[8, 20, 25, 28]);
        assert_eq!(result, 28);
    }
}
