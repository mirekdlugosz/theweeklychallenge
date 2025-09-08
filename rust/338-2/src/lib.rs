fn arr_min_max(arr: &[isize]) -> (isize, isize) {
    let mut arr_min = isize::MAX;
    let mut arr_max = isize::MIN;

    for i in arr {
        if arr_min > *i {
            arr_min = *i;
        }
        if *i > arr_max {
            arr_max = *i;
        }
    }
    (arr_min, arr_max)
}

// Exercise description is a bit strange, as it painstakingly shows the absolute difference
// between each pair of numbers. But this is not necessary at all. The maximum distance is
// between smallest number from one array and largest number from other array. The only problem
// is that we don't know which array has which number. So we iterate through each array, looking
// for both smallest and largest number. Then we compare smallest from array1 to largest from
// array2, and smallest from array2 to largest from array1. Then we take whichever of these two
// numbers is larger.
pub fn max_distance(arr1: &[isize], arr2: &[isize]) -> usize {
    let (arr1_min, arr1_max) = arr_min_max(arr1);
    let (arr2_min, arr2_max) = arr_min_max(arr2);
    let candidate1 = arr1_max.abs_diff(arr2_min);
    let candidate2 = arr2_max.abs_diff(arr1_min);
    candidate1.max(candidate2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = max_distance(&[4, 5, 7], &[9, 1, 3, 4]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test2() {
        let result = max_distance(&[2, 3, 5, 4], &[3, 2, 5, 5, 8, 7]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test3() {
        let result = max_distance(&[2, 1, 11, 3], &[2, 5, 10, 2]);
        assert_eq!(result, 9);
    }

    #[test]
    fn test4() {
        let result = max_distance(&[1, 2, 3], &[3, 2, 1]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test5() {
        let result = max_distance(&[1, 0, 2, 3], &[5, 0]);
        assert_eq!(result, 5);
    }
}
