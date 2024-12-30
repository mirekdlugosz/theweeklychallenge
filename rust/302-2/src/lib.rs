pub fn step_by_step(ints: &[isize]) -> isize {
    let mut tracker: isize = 0;
    let mut lowest: isize = isize::MAX;
    for number in ints {
        tracker += number;
        lowest = lowest.min(tracker);
    }
    1.max(1 - lowest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = step_by_step(&[-3, 2, -3, 4, 2]);
        assert_eq!(result, 5);
    }

    #[test]
    fn test2() {
        let result = step_by_step(&[1, 2]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let result = step_by_step(&[1, -2, -3]);
        assert_eq!(result, 5);
    }
}
