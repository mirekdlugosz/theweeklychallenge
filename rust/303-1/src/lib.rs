use std::collections::HashSet;

use itertools::Itertools;

pub fn three_digit_even(ints: &[usize]) -> HashSet<u16> {
    ints.iter()
        .permutations(3)
        .unique()
        .filter(|permutation| {
            let first = **(permutation.first().unwrap_or(&&0));
            let last = permutation.last().unwrap_or(&&0);
            first != 0 && last.rem_euclid(2) == 0
        })
        .filter_map(|vec| {
            vec.iter().join("").parse().ok()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = three_digit_even(&[2, 1, 3, 0]);
        let expected: HashSet<u16> = [102, 120, 130, 132, 210, 230, 302, 310, 312, 320].into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = three_digit_even(&[2, 2, 8, 8, 2]);
        let expected: HashSet<u16> = [222, 228, 282, 288, 822, 828, 882].into();
        assert_eq!(result, expected);
    }
}
