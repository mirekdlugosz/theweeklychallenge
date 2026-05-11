use std::iter::repeat_n;

pub fn list_division(list: &[usize], n: usize) -> Vec<Vec<usize>> {
    let list_length = list.len();
    let bucket_size = list_length.div_euclid(n);
    let larger_buckets_n = list_length.rem_euclid(n);
    let larger_bucket_size = bucket_size.saturating_add(1);
    let mut out = Vec::with_capacity(n);
    let mut list_iter = list.iter();
    let bucket_sizes = repeat_n(larger_bucket_size, larger_buckets_n)
        .chain(repeat_n(bucket_size, n.saturating_sub(larger_buckets_n)));

    for single_bucket_size in bucket_sizes {
        let elem: Vec<usize> = list_iter
            .by_ref()
            .take(single_bucket_size)
            .copied()
            .collect();
        out.push(elem);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = list_division(&[1, 2, 3, 4, 5], 2);
        let expected = vec![vec![1, 2, 3], vec![4, 5]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = list_division(&[1, 2, 3, 4, 5, 6], 3);
        let expected = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = list_division(&[1, 2, 3], 2);
        let expected = vec![vec![1, 2], vec![3]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let result = list_division(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5);
        let expected = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let result = list_division(&[72, 57, 89, 55, 36, 84, 10, 95, 99, 35], 7);
        let expected = vec![
            vec![72, 57],
            vec![89, 55],
            vec![36, 84],
            vec![10],
            vec![95],
            vec![99],
            vec![35],
        ];
        assert_eq!(result, expected);
    }
}
