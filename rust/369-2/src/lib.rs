use std::iter::repeat_n;

// The idea is to depend on chunks_exact which will return chunks of specified
// length and ignore remainder.
// We blindly append filler character to the string so the last chunk has them.
// But we must be cautious to not end up with a chunk that is all filler
// characters, hence repeat_n with one less thank chunk size.
pub fn group_division(str: &str, size: usize, filler: char) -> Vec<String> {
    str.chars()
        .chain(repeat_n(filler, size.saturating_sub(1)))
        .collect::<Vec<_>>()
        .chunks_exact(size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = group_division("RakuPerl", 4, '*');
        assert_eq!(result, ["Raku", "Perl"]);
    }

    #[test]
    fn test2() {
        let result = group_division("Python", 5, '0');
        assert_eq!(result, ["Pytho", "n0000"]);
    }

    #[test]
    fn test3() {
        let result = group_division("12345", 3, 'x');
        assert_eq!(result, ["123", "45x"]);
    }

    #[test]
    fn test4() {
        let result = group_division("HelloWorld", 3, '_');
        assert_eq!(result, ["Hel", "loW", "orl", "d__"]);
    }

    #[test]
    fn test5() {
        let result = group_division("AI", 5, '!');
        assert_eq!(result, ["AI!!!"]);
    }
}
