// Fundamentally, this is so-called "multiway number partitioning" problem
// with number of bins k=4
// Wikipedia describes a number of algorithms with various performance and
// complexity profiles. Below is the most basic implementation of greedy
// number partitioning. It does not guarantee to find the optimal sum, which
// I guess means it might return false instead of true for some inputs?

pub fn matchstick_square(ints: &[usize]) -> bool {
    let mut bins: [usize; 4] = [0, 0, 0, 0];

    for number in ints {
        // first, find the bin that has the smallest number
        let mut min = usize::MAX;
        let mut min_idx = 0;
        for (bin_idx, bin_val) in bins.iter().enumerate() {
            if &min > bin_val {
                min_idx = bin_idx;
                min = *bin_val;
            }
        }

        // second, add current number to found bin
        if let Some(new_val) = bins.get_mut(min_idx) {
            *new_val += number;
        }
    }

    // check if all bins have the same value
    bins.windows(2).all(|chunk| chunk[0] == chunk[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = matchstick_square(&[1, 2, 2, 2, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = matchstick_square(&[2, 2, 2, 4]);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = matchstick_square(&[2, 2, 2, 2, 4]);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = matchstick_square(&[3, 4, 1, 4, 3, 1]);
        assert_eq!(result, true);
    }
}
