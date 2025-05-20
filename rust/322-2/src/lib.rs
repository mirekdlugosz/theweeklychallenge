use std::collections::HashSet;

pub fn rank_array(ints: &[usize]) -> Vec<usize> {
    let mut sorted: Vec<&usize> = ints
        .iter()
        .collect::<HashSet<&usize>>()
        .into_iter()
        .collect();
    sorted.sort_unstable();
    ints.iter()
        .map(|i| sorted.iter().position(|&j| i == j).unwrap() + 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = rank_array(&[55, 22, 44, 33]);
        assert_eq!(result, vec!(4, 1, 3, 2));
    }

    #[test]
    fn test2() {
        let result = rank_array(&[10, 10, 10]);
        assert_eq!(result, vec!(1, 1, 1));
    }

    #[test]
    fn test3() {
        let result = rank_array(&[5, 1, 1, 4, 3]);
        assert_eq!(result, vec!(4, 1, 1, 3, 2));
    }
}
