use std::collections::HashSet;

pub fn arrays_intersection(arrays: &[&[isize]]) -> HashSet<isize> {
    let first = arrays.first().unwrap();
    let mut output: HashSet<isize> = first.iter().copied().collect();

    let remaining = arrays.iter().skip(1);
    for array in remaining {
        let as_hash: HashSet<isize> = array.iter().copied().collect();
        output = output.intersection(&as_hash).copied().collect();
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = arrays_intersection(&[&[1, 2, 3, 4], &[4, 5, 6, 1], &[4, 2, 1, 3]]);
        assert_eq!(result, [1, 4].into());
    }

    #[test]
    fn test2() {
        let result = arrays_intersection(&[&[1, 0, 2, 3], &[2, 4, 5]]);
        assert_eq!(result, [2].into());
    }

    #[test]
    fn test3() {
        let result = arrays_intersection(&[&[1, 2, 3], &[4, 5], &[6]]);
        assert_eq!(result, [].into());
    }
}
