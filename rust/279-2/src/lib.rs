static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn split_string(input: &str) -> bool {
    // the exercise explicitly calls out splitting a string into two,
    // but since the output is only a boolean specifying if that can
    // be done at all, we might as well skip splitting. If number of
    // vowels is even, then string can be split into two containing
    // the same number of vowels each. Otherwise it can't.
    let vowels_num = input
        .to_lowercase()
        .chars()
        .filter(|ch| VOWELS.contains(ch))
        .count();

    vowels_num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = split_string("perl");
        assert_eq!(result, false);
    }

    #[test]
    fn test2() {
        let result = split_string("book");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = split_string("good morning");
        assert_eq!(result, true);
    }
}
