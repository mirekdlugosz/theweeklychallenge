use std::collections::HashSet;

pub fn minimum_common(array1: &[usize], array2: &[usize]) -> Option<usize> {
    let hash1: HashSet<usize> = array1.iter().copied().collect();
    let hash2: HashSet<usize> = array2.iter().copied().collect();
    hash1.intersection(&hash2).min().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = minimum_common(&[1, 2, 3, 4], &[3, 4, 5, 6]);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test2() {
        let result = minimum_common(&[1, 2, 3], &[2, 4]);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test3() {
        let result = minimum_common(&[1, 2, 3, 4], &[5, 6, 7, 8]);
        assert_eq!(result, None);
    }
}
