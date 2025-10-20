use itertools::Itertools;

pub fn array_formation(source: &[&Vec<isize>], target: &[isize]) -> bool {
    let total_source_len: usize = source.iter().map(|i| i.len()).sum();
    let target_len = target.len();
    if total_source_len != target_len {
        return false;
    }
    let source_len = source.len();
    source.iter().permutations(source_len).any(|perm| {
        let flattened: Vec<isize> = perm.iter().flat_map(|i| i.iter()).copied().collect();
        flattened == target
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let source = &[&vec![2, 3], &vec![1], &vec![4]];
        let target = &[1, 2, 3, 4];
        let result = array_formation(source, target);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let source = &[&vec![1, 3], &vec![2, 4]];
        let target = &[1, 2, 3, 4];
        let result = array_formation(source, target);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let source = &[&vec![9, 1], &vec![5, 8], &vec![2]];
        let target = &[5, 8, 2, 9, 1];
        let result = array_formation(source, target);
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let source = &[&vec![1], &vec![3]];
        let target = &[1, 2, 3];
        let result = array_formation(source, target);
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let source = &[&vec![7, 4, 6]];
        let target = &[7, 4, 6];
        let result = array_formation(source, target);
        assert_eq!(result, true);
    }
}
