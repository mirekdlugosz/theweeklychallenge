pub fn target_index(ints: &[isize], k: isize) -> Vec<usize> {
    let mut sorted_ints = ints.to_vec();
    sorted_ints.sort_unstable();
    sorted_ints
        .iter()
        .enumerate()
        .filter_map(|(idx, n)| {
            match *n == k {
                true => Some(idx),
                false => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ints = vec!(1, 5, 3, 2, 4, 2);
        assert_eq!(target_index(&ints, 2), vec!(1, 2));
    }

    #[test]
    fn test2() {
        let ints = vec!(1, 2, 4, 3, 5);
        assert_eq!(target_index(&ints, 6), vec!());
    }

    #[test]
    fn test3() {
        let ints = vec!(5, 3, 2, 4, 2, 1);
        assert_eq!(target_index(&ints, 4), vec!(4));
    }
}
