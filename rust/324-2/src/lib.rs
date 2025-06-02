use itertools::Itertools;

pub fn total_xor(ints: &[isize]) -> isize {
    let max_k = ints.len();
    (1..=max_k)
        .flat_map(|k| ints.iter().copied().combinations(k).collect::<Vec<_>>())
        .map(|tuple| tuple.iter().copied().reduce(|acc, e| acc ^ e).unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = total_xor(&[1, 3]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test2() {
        let result = total_xor(&[5, 1, 6]);
        assert_eq!(result, 28);
    }

    #[test]
    fn test3() {
        let result = total_xor(&[3, 4, 5, 6, 7, 8]);
        assert_eq!(result, 480);
    }
}
