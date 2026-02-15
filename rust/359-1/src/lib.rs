fn get_digits(int: usize) -> Vec<usize> {
    if int == 0 {
        return vec![0];
    }

    let mut k = int;
    let mut out = Vec::new();
    while k > 0 {
        out.push(k % 10);
        k /= 10;
    }
    out.reverse();
    out
}

pub fn digital_root(int: usize) -> (usize, usize) {
    let mut persistence = 0;
    let mut digit_root = int;

    while digit_root > 9 {
        let digits = get_digits(digit_root);
        digit_root = digits.iter().sum();
        persistence += 1;
    }

    (persistence, digit_root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = digital_root(38);
        assert_eq!(result, (2, 2));
    }

    #[test]
    fn test2() {
        let result = digital_root(7);
        assert_eq!(result, (0, 7));
    }

    #[test]
    fn test3() {
        let result = digital_root(999);
        assert_eq!(result, (2, 9));
    }

    #[test]
    fn test4() {
        let result = digital_root(1999999999);
        assert_eq!(result, (3, 1));
    }

    #[test]
    fn test5() {
        let result = digital_root(101010);
        assert_eq!(result, (1, 3));
    }
}
