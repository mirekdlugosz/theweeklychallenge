pub fn count_asterisks(input: &str) -> usize {
    input
        .split('|')
        .step_by(2)
        .map(|elem| {
            elem
                .chars()
                .filter(|ch| ch == &'*')
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = count_asterisks("p|*e*rl|w**e|*ekly|");
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = count_asterisks("perl");
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let result = count_asterisks("th|ewe|e**|k|l***ych|alleng|e");
        assert_eq!(result, 5);
    }
}
