pub fn ascending_numbers(str: &str) -> bool {
    let tmp: Vec<i32> = str
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect();
    tmp.windows(2)
        .all(|chunk| chunk.first().unwrap_or(&0) < chunk.last().unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = ascending_numbers("The cat has 3 kittens 7 toys 10 beds");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = ascending_numbers("Alice bought 5 apples 2 oranges 9 bananas");
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = ascending_numbers("I ran 1 mile 2 days 3 weeks 4 months");
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let result = ascending_numbers("Bob has 10 cars 10 bikes");
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let result = ascending_numbers("Zero is 0 one is 1 two is 2");
        assert_eq!(result, true);
    }
}
