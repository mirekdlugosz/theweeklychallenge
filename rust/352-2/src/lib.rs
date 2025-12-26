pub fn binary_prefix(nums: &[usize]) -> Vec<bool> {
    nums.iter()
        .enumerate()
        .map(|(idx, _)| {
            let mut binary_str = String::new();
            for e in nums.get(0..=idx).unwrap() {
                binary_str.push_str(&e.to_string());
            }
            let number = isize::from_str_radix(&binary_str, 2).unwrap();
            number % 5 == 0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = binary_prefix(&[0, 1, 1, 0, 0, 1, 0, 1, 1, 1]);
        let expected = vec![
            true, false, false, false, false, true, true, false, false, false,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = binary_prefix(&[1, 0, 1, 0, 1, 0]);
        let expected = vec![false, false, true, true, false, false];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = binary_prefix(&[0, 0, 1, 0, 1]);
        let expected = vec![true, true, false, false, true];
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let result = binary_prefix(&[1, 1, 1, 1, 1]);
        let expected = vec![false, false, false, true, false];
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let result = binary_prefix(&[1, 0, 1, 1, 0, 1, 0, 0, 1, 1]);
        let expected = vec![
            false, false, true, false, false, true, true, true, false, false,
        ];
        assert_eq!(result, expected);
    }
}
