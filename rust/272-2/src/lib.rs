pub fn string_score(input: &str) -> usize {
    input
        .chars()
        .map(|ch| ch as u32)
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|values| {
            let diff = values[0].abs_diff(values[1]);
            diff as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(string_score("hello"), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(string_score("perl"), 30);
    }

    #[test]
    fn test3() {
        assert_eq!(string_score("raku"), 37);
    }

    #[test]
    fn test4() {
        assert_eq!(string_score("rust"), 6);
    }
}
