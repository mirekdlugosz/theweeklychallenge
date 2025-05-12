use ordered_float::OrderedFloat;
use std::collections::{HashSet, VecDeque};

pub fn distinct_average(nums: &[isize]) -> usize {
    let mut averages: HashSet<OrderedFloat<f64>> = HashSet::with_capacity(nums.len() / 2);
    let mut sorted: VecDeque<isize> = nums.iter().copied().collect();
    sorted.make_contiguous().sort_unstable();

    while !sorted.is_empty() {
        let first = sorted.pop_front().unwrap();
        let last = sorted.pop_back().unwrap();
        let avg = OrderedFloat((first as f64 + last as f64) / 2.0);
        averages.insert(avg);
    }
    averages.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = distinct_average(&[1, 2, 4, 3, 5, 6]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = distinct_average(&[0, 2, 4, 8, 3, 5]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let result = distinct_average(&[7, 3, 1, 0, 5, 9]);
        assert_eq!(result, 2);
    }
}
