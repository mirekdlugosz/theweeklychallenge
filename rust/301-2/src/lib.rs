fn hamming_distance(left: u64, right: u64) -> u64 {
    // that's a bit wasteful, but we need to ensure both
    // strings are the same length. Since we made input
    // numbers u64, we might as well make strings that long
    let b_left = format!("{left:064b}");
    let b_right = format!("{right:064b}");

    b_left.chars()
        .zip(b_right.chars())
        .filter(|(l, r)| l != r)
        .count()
        .try_into()
        .unwrap_or(0)
}

pub fn hamming_distance_sum(ints: &[u64]) -> u64 {
    let mut sum: u64 = 0;
    for (idx, i) in ints.iter().enumerate() {
        for j in ints.iter().skip(idx) {
            sum += hamming_distance(*i, *j);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = hamming_distance_sum(&[4, 14, 2]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test2() {
        let result = hamming_distance_sum(&[4, 14, 4]);
        assert_eq!(result, 4);
    }
}
