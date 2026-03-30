use std::iter::repeat_n;

pub fn max_odd_binary(str: &str) -> String {
    let mut n_zeroes: usize = 0;
    let mut n_ones: usize = 0;
    for ch in str.chars() {
        match ch {
            '0' => n_zeroes += 1,
            '1' => n_ones += 1,
            _ => (),
        }
    }
    repeat_n('1', n_ones.saturating_sub(1))
        .chain(repeat_n('0', n_zeroes))
        .chain(repeat_n('1', 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = max_odd_binary("1011");
        assert_eq!(result, "1101");
    }

    #[test]
    fn test2() {
        let result = max_odd_binary("100");
        assert_eq!(result, "001");
    }

    #[test]
    fn test3() {
        let result = max_odd_binary("111000");
        assert_eq!(result, "110001");
    }

    #[test]
    fn test4() {
        let result = max_odd_binary("0101");
        assert_eq!(result, "1001");
    }

    #[test]
    fn test5() {
        let result = max_odd_binary("1111");
        assert_eq!(result, "1111");
    }
}
