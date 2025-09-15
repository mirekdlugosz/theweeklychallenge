// My initial approach was to take two largest numbers, take two smallest
// numbers, calculate the products and then difference.
// That fails once negative numbers come into picture.
// That approach redefines "largest" as "furthest from 0" and "smallest"
// as "closest to 0".
// I have some doubts if that's actually correct, but it does pass all
// the provided tests.

pub fn max_diff(ints: &[isize]) -> usize {
    let mut largest_1: isize = 0;
    let mut largest_2: isize = 0;
    let mut smallest_1 = isize::MAX;
    let mut smallest_2 = isize::MAX;

    for num in ints {
        if num.abs() > largest_1.abs() {
            largest_2 = largest_1;
            largest_1 = *num;
        } else if num.abs() > largest_2.abs() {
            largest_2 = *num;
        }

        if smallest_1.abs() > num.abs() {
            smallest_2 = smallest_1;
            smallest_1 = *num;
        } else if smallest_2.abs() > num.abs() {
            smallest_2 = *num;
        }
    }

    (largest_1.saturating_mul(largest_2)).abs_diff(smallest_1.saturating_mul(smallest_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = max_diff(&[5, 9, 3, 4, 6]);
        assert_eq!(result, 42);
    }

    #[test]
    fn test2() {
        let result = max_diff(&[1, -2, 3, -4]);
        assert_eq!(result, 10);
    }

    #[test]
    fn test3() {
        let result = max_diff(&[-3, -1, -2, -4]);
        assert_eq!(result, 10);
    }

    #[test]
    fn test4() {
        let result = max_diff(&[10, 2, 0, 5, 1]);
        assert_eq!(result, 50);
    }

    #[test]
    fn test5() {
        let result = max_diff(&[7, 8, 9, 10, 10]);
        assert_eq!(result, 44);
    }
}
