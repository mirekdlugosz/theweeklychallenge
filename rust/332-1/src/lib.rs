pub fn binary_date(date: &str) -> String {
    date.split('-')
        .filter_map(|num| {
            if let Ok(parsed) = num.parse::<usize>() {
                return Some(format!("{parsed:b}"));
            }
            None
        })
        .collect::<Vec<String>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = binary_date("2025-07-26");
        assert_eq!(result, "11111101001-111-11010");
    }

    #[test]
    fn test2() {
        let result = binary_date("2000-02-02");
        assert_eq!(result, "11111010000-10-10");
    }

    #[test]
    fn test3() {
        let result = binary_date("2024-12-31");
        assert_eq!(result, "11111101000-1100-11111");
    }
}
