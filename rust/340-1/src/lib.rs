pub fn duplicate_removals(str: &str) -> String {
    let mut output: Vec<_> = str.chars().collect();
    'main: loop {
        let mut iter = output.iter().enumerate().peekable();
        while let Some((idx, letter)) = iter.next()
            && let Some((next_idx, next_letter)) = iter.peek()
        {
            if letter == *next_letter {
                output.remove(*next_idx);
                output.remove(idx);
                continue 'main;
            }
        }
        break;
    }
    output.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = duplicate_removals("abbaca");
        assert_eq!(result, "ca");
    }

    #[test]
    fn test2() {
        let result = duplicate_removals("azxxzy");
        assert_eq!(result, "ay");
    }

    #[test]
    fn test3() {
        let result = duplicate_removals("aaaaaaaa");
        assert_eq!(result, "");
    }

    #[test]
    fn test4() {
        let result = duplicate_removals("aabccba");
        assert_eq!(result, "a");
    }

    #[test]
    fn test5() {
        let result = duplicate_removals("abcddcba");
        assert_eq!(result, "");
    }
}
