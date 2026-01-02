use std::{cmp::Ordering, collections::HashSet};

fn as_ordered_tuple(i: isize, j: isize) -> (isize, isize) {
    if i > j { (j, i) } else { (i, j) }
}

pub fn min_abs_diff(ints: &[isize]) -> HashSet<(isize, isize)> {
    let mut output: HashSet<(isize, isize)> = HashSet::with_capacity(ints.len());
    let mut min_diff: usize = usize::MAX;
    for (idx, i) in ints.iter().enumerate() {
        for j in ints.iter().skip(idx + 1) {
            let diff = i.abs_diff(*j);
            match diff.cmp(&min_diff) {
                Ordering::Greater => (),
                Ordering::Equal => {
                    output.insert(as_ordered_tuple(*i, *j));
                }
                Ordering::Less => {
                    min_diff = diff;
                    output.clear();
                    output.insert(as_ordered_tuple(*i, *j));
                }
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = min_abs_diff(&[4, 2, 1, 3]);
        let expected: HashSet<(isize, isize), _> = [(1, 2), (2, 3), (3, 4)].into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = min_abs_diff(&[10, 100, 20, 30]);
        let expected: HashSet<(isize, isize), _> = [(10, 20), (20, 30)].into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = min_abs_diff(&[-5, -2, 0, 3]);
        let expected: HashSet<(isize, isize), _> = [(-2, 0)].into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let result = min_abs_diff(&[8, 1, 15, 3]);
        let expected: HashSet<(isize, isize), _> = [(1, 3)].into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let result = min_abs_diff(&[12, 5, 9, 1, 15]);
        let expected: HashSet<(isize, isize), _> = [(9, 12), (12, 15)].into();
        assert_eq!(result, expected);
    }
}
