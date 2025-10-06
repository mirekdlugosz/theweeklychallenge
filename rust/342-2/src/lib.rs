pub fn max_score(str: &str) -> usize {
    str.char_indices()
        .skip(1)
        .take(str.chars().count() - 1)
        .map(|(idx, _)| {
            let (left, right) = str.split_at(idx);
            left.chars().filter(|&c| c == '0').count() + right.chars().filter(|&c| c == '1').count()
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = max_score("0011");
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let result = max_score("0000");
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let result = max_score("1111");
        assert_eq!(result, 3);
    }

    #[test]
    fn test4() {
        let result = max_score("0101");
        assert_eq!(result, 3);
    }

    #[test]
    fn test5() {
        let result = max_score("011101");
        assert_eq!(result, 5);
    }

    #[test]
    fn test6() {
        let result = max_score("010001");
        assert_eq!(result, 5);
    }
}
