pub fn final_price(prices: &[usize]) -> Vec<usize> {
    prices
        .iter()
        .enumerate()
        .map(|(idx, value)| {
            let next_smaller = prices
                .iter()
                .skip(idx + 1)
                .find(|e| value >= e)
                .unwrap_or(&0);
            value - next_smaller
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = final_price(&[8, 4, 6, 2, 3]);
        assert_eq!(result, vec!(4, 2, 4, 2, 3));
    }

    #[test]
    fn test2() {
        let result = final_price(&[1, 2, 3, 4, 5]);
        assert_eq!(result, vec!(1, 2, 3, 4, 5));
    }
    #[test]
    fn test3() {
        let result = final_price(&[7, 1, 1, 5]);
        assert_eq!(result, vec!(6, 0, 1, 5));
    }
}
