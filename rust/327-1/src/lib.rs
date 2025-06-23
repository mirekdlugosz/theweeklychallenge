use std::collections::HashSet;

pub fn missing_integers(ints: &[usize]) -> Vec<usize> {
    let known_ints: HashSet<&usize> = ints.iter().collect();
    let n = ints.len();
    let mut missing = Vec::with_capacity(n);

    for i in 1..=n {
        if !known_ints.contains(&i) {
            missing.push(i);
        }
    }

    missing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = missing_integers(&[1, 2, 1, 3, 2, 5]);
        assert_eq!(result, vec![4, 6]);
    }

    #[test]
    fn test2() {
        let result = missing_integers(&[1, 1, 1]);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test3() {
        let result = missing_integers(&[2, 2, 1]);
        assert_eq!(result, vec![3]);
    }
}
