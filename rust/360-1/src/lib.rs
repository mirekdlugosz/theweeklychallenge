use std::iter::repeat_n;

const PAD_CHAR: char = '*';

pub fn text_justifier(str: &str, width: usize) -> String {
    let padding = width - str.chars().count();
    let half_padding = padding.div_euclid(2);
    let remainder = padding.rem_euclid(2);
    repeat_n(PAD_CHAR, half_padding)
        .chain(str.chars())
        .chain(repeat_n(PAD_CHAR, half_padding + remainder))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = text_justifier("Hi", 5);
        assert_eq!(result, "*Hi**");
    }

    #[test]
    fn test2() {
        let result = text_justifier("Code", 10);
        assert_eq!(result, "***Code***");
    }

    #[test]
    fn test3() {
        let result = text_justifier("Hello", 9);
        assert_eq!(result, "**Hello**");
    }

    #[test]
    fn test4() {
        let result = text_justifier("Perl", 4);
        assert_eq!(result, "Perl");
    }

    #[test]
    fn test5() {
        let result = text_justifier("A", 7);
        assert_eq!(result, "***A***");
    }

    #[test]
    fn test6() {
        let result = text_justifier("", 5);
        assert_eq!(result, "*****");
    }
}
