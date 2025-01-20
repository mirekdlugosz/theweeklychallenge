fn is_prime(number: isize) -> bool {
    if number <= 1 {
        return false;
    }

    for i in 2..=((number as f64).sqrt() as isize) {
        if number % i == 0 {
            return false;
        }
    }

    true
}

pub fn binary_prefix(binary: &[usize]) -> Vec<bool> {
    binary
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let mut binary_str = String::new();
            for e in binary.get(0..=idx).unwrap() {
                binary_str.push_str(&e.to_string());
            }
            let number = isize::from_str_radix(&binary_str, 2).unwrap();
            is_prime(number)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = binary_prefix(&[1, 0, 1]);
        let expected = vec![false, true, true];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = binary_prefix(&[1, 1, 0]);
        let expected = vec![false, true, false];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = binary_prefix(&[1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1]);
        let expected = vec![
            false, true, true, false, false, true, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true,
        ];
        assert_eq!(result, expected);
    }
}
