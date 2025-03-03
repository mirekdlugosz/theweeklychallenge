pub fn group_digit_sum(numbers: &str, int: usize) -> String {
    let mut new_numbers: String = numbers.chars().collect();

    while new_numbers.len() > int {
        new_numbers = new_numbers
            .chars()
            .collect::<Vec<_>>()
            .chunks(int)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|ch| ch.to_string().as_str().parse::<isize>().unwrap_or(0))
                    .sum::<isize>()
                    .to_string()
            })
            .collect();
    }

    new_numbers.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = group_digit_sum("111122333", 3);
        assert_eq!(result, "359".to_string());
    }

    #[test]
    fn test2() {
        let result = group_digit_sum("1222312", 2);
        assert_eq!(result, "76".to_string());
    }

    #[test]
    fn test3() {
        let result = group_digit_sum("100012121001", 4);
        assert_eq!(result, "162".to_string());
    }
}
