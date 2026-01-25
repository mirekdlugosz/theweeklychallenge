fn as_digits(number: usize) -> (usize, usize, usize, usize) {
    (
        number / 1000,
        (number / 100) % 10,
        (number / 10) % 10,
        number % 10,
    )
}

fn as_int(digits: Vec<usize>) -> usize {
    let mut i = digits.iter();
    let thousands = i.next().unwrap_or(&0);
    let hundreds = i.next().unwrap_or(&0);
    let tens = i.next().unwrap_or(&0);
    let ones = i.next().unwrap_or(&0);
    thousands * 1000 + hundreds * 100 + tens * 10 + ones
}

pub fn kaprekar_constant(number: usize) -> Option<u8> {
    let digits = as_digits(number);
    if digits.0 == digits.1 && digits.1 == digits.2 && digits.2 == digits.3 {
        return None;
    }

    let mut iterations: u8 = 0;
    let mut current_diff = number;
    while current_diff != 6174 {
        let digits = as_digits(current_diff);
        let mut ascending = vec![digits.0, digits.1, digits.2, digits.3];
        ascending.sort_unstable();
        let mut descending = ascending.to_vec();
        descending.reverse();

        current_diff = as_int(descending) - as_int(ascending);

        iterations += 1;
    }
    Some(iterations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = kaprekar_constant(3524);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test2() {
        let result = kaprekar_constant(6174);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test3() {
        let result = kaprekar_constant(9998);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test4() {
        let result = kaprekar_constant(1001);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test5() {
        let result = kaprekar_constant(9000);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test6() {
        let result = kaprekar_constant(1111);
        assert_eq!(result, None);
    }
}
