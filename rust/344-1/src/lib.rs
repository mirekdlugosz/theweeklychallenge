pub fn array_from_compute(ints: &[usize], x: usize) -> Vec<usize> {
    let number: usize = ints
        .iter()
        .map(usize::to_string)
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    let new_number = number.saturating_add(x);
    new_number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0) as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = array_from_compute(&[1, 2, 3, 4], 12);
        assert_eq!(result, vec![1, 2, 4, 6]);
    }

    #[test]
    fn test2() {
        let result = array_from_compute(&[2, 7, 4], 181);
        assert_eq!(result, vec![4, 5, 5]);
    }

    #[test]
    fn test3() {
        let result = array_from_compute(&[9, 9, 9], 1);
        assert_eq!(result, vec![1, 0, 0, 0]);
    }

    #[test]
    fn test4() {
        let result = array_from_compute(&[1, 0, 0, 0, 0], 9999);
        assert_eq!(result, vec![1, 9, 9, 9, 9]);
    }

    #[test]
    fn test5() {
        let result = array_from_compute(&[0], 1000);
        assert_eq!(result, vec![1, 0, 0, 0]);
    }
}
