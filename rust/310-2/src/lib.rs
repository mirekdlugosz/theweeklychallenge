use itertools::{Either, Itertools};

pub fn odd_order(ints: &[isize]) -> Vec<isize> {
    let (mut odd, mut even): (Vec<_>, Vec<_>) =
        ints.iter().enumerate().partition_map(|(idx, x)| {
            if idx.rem_euclid(2) == 1 {
                Either::Left(*x)
            } else {
                Either::Right(*x)
            }
        });
    even.sort_unstable();
    odd.sort_unstable();
    odd.reverse();
    even.iter().interleave(&odd).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = odd_order(&[4, 1, 2, 3]);
        let expected = [2, 3, 4, 1].to_vec();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = odd_order(&[3, 1]);
        let expected = [3, 1].to_vec();
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = odd_order(&[5, 3, 2, 1, 4]);
        let expected = [2, 3, 4, 1, 5].to_vec();
        assert_eq!(result, expected);
    }
}
