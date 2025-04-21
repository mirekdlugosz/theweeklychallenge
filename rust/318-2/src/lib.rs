pub fn reverse_equals(source: &[usize], target: &[usize]) -> bool {
    if source.len() != target.len() {
        return false;
    }

    if source.len() == 1 {
        return source == target;
    }

    let mut diffs: Vec<usize> = Vec::new();

    for (idx, (s, t)) in source.iter().zip(target.iter()).enumerate() {
        if s != t {
            diffs.push(idx);
        }
    }

    let first_diff = *diffs.first().unwrap();
    let last_diff = *diffs.last().unwrap();

    let mut source_part = source
        .get(first_diff..=last_diff)
        .map_or(Vec::new(), |s| s.to_vec());
    let target_part = target
        .get(first_diff..=last_diff)
        .map_or(Vec::new(), |s| s.to_vec());

    source_part.reverse();

    source_part == target_part
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = reverse_equals(&[3, 2, 1, 4], &[1, 2, 3, 4]);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = reverse_equals(&[1, 3, 4], &[4, 1, 3]);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = reverse_equals(&[2], &[2]);
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let result = reverse_equals(&[1, 2, 4, 3, 5], &[1, 2, 3, 4, 5]);
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = reverse_equals(&[3, 2, 1], &[1, 2, 3]);
        assert_eq!(result, true);
    }

    #[test]
    fn test6() {
        let result = reverse_equals(&[2], &[6]);
        assert_eq!(result, false);
    }
}
