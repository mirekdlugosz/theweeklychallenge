use itertools::{self, Itertools};
use std::collections::HashSet;

fn subset_sum(subset: &[(usize, &isize)]) -> bool {
    let sums = subset
        .iter()
        .fold((0, 0), |acc, elem| (acc.0 + elem.0 + 1, acc.1 + elem.1));
    sums.0 == sums.1.try_into().unwrap()
}

pub fn subset_equilibrium(nums: &[isize]) -> HashSet<Vec<isize>> {
    let total_elems = nums.len();
    let mut hash: HashSet<Vec<isize>> = HashSet::with_capacity(total_elems);
    for subset in nums.iter().enumerate().powerset() {
        if 1 >= subset.len() || nums.len() == subset.len() {
            continue;
        }
        if subset_sum(&subset) {
            let items = subset.iter().map(|elem| *elem.1).collect();
            hash.insert(items);
        }
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = subset_equilibrium(&[2, 1, 4, 3]);
        let expected = HashSet::from_iter([vec![2, 1], vec![1, 4], vec![4, 3], vec![2, 3]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = subset_equilibrium(&[3, 0, 3, 0]);
        let expected = HashSet::from_iter([vec![3, 0], vec![3, 0, 3]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = subset_equilibrium(&[5, 1, 1, 1]);
        let expected = HashSet::from_iter([vec![5, 1, 1]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let result = subset_equilibrium(&[3, -1, 4, 2]);
        let expected = HashSet::from_iter([vec![3, 2], vec![3, -1, 4]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let result = subset_equilibrium(&[10, 20, 30, 40]);
        let expected = HashSet::from_iter([]);
        assert_eq!(result, expected);
    }
}
