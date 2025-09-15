pub fn peak_point(gain: &[isize]) -> isize {
    let mut current = 0;
    let mut highest = 0;
    for change in gain {
        current += *change;
        if current > highest {
            highest = current;
        }
    }
    highest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = peak_point(&[-5, 1, 5, -9, 2]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = peak_point(&[10, 10, 10, -25]);
        assert_eq!(result, 30);
    }

    #[test]
    fn test3() {
        let result = peak_point(&[3, -4, 2, 5, -6, 1]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test4() {
        let result = peak_point(&[-1, -2, -3, -4]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test5() {
        let result = peak_point(&[-10, 15, 5]);
        assert_eq!(result, 10);
    }
}
