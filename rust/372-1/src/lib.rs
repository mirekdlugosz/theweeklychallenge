pub fn rearrange_spaces(str: &str) -> String {
    let spaces = str.chars().filter(|ch| ch.is_whitespace()).count();
    let words: Vec<_> = str.split_whitespace().collect();
    let gaps = words.len().saturating_sub(1);
    let (gap_spaces, remainder_spaces) = if gaps > 0 {
        (spaces.div_euclid(gaps), spaces.rem_euclid(gaps))
    } else {
        (0, spaces)
    };
    let o: String = words.join(&" ".repeat(gap_spaces));
    format!("{o}{}", " ".repeat(remainder_spaces))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = rearrange_spaces("  challenge  ");
        assert_eq!(result, "challenge    ");
    }

    #[test]
    fn test2() {
        let result = rearrange_spaces("coding  is  fun");
        assert_eq!(result, "coding  is  fun");
    }

    #[test]
    fn test3() {
        let result = rearrange_spaces("a b c  d");
        assert_eq!(result, "a b c d ");
    }

    #[test]
    fn test4() {
        let result = rearrange_spaces("  team      pwc  ");
        assert_eq!(result, "team          pwc");
    }

    #[test]
    fn test5() {
        let result = rearrange_spaces("   the  weekly  challenge  ");
        assert_eq!(result, "the    weekly    challenge ");
    }
}
