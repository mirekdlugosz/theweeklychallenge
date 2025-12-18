pub fn arithmetic_progression(num: &[f32]) -> bool {
    let mut sorted: Vec<f32> = num.to_vec();
    sorted.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let first = sorted.first().unwrap_or(&0.0);
    let second = sorted.get(1).unwrap_or(&0.0);
    let diff = *second - *first;
    sorted.windows(2).all(|slice| {
        let slice_first = slice.first().unwrap_or(&0.0);
        let slice_last = slice.last().unwrap_or(&0.0);
        (*slice_last - *slice_first) == diff
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = arithmetic_progression(&[1.0, 3.0, 5.0, 7.0, 9.0]);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = arithmetic_progression(&[9.0, 1.0, 7.0, 5.0, 3.0]);
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = arithmetic_progression(&[1.0, 2.0, 4.0, 8.0, 16.0]);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = arithmetic_progression(&[5.0, -1.0, 3.0, 1.0, -3.0]);
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = arithmetic_progression(&[1.5, 3.0, 0.0, 4.5, 6.0]);
        assert_eq!(result, true);
    }
}
