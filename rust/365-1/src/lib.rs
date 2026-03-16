use std::fmt::Write;

static LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn alphabet_index_digit_sum(str: &str, k: usize) -> usize {
    let mut out = 0;
    let mut digits: String = str.chars().fold(String::new(), |mut output, c| {
        let num = LETTERS.iter().position(|&l| l == c).unwrap() + 1;
        let _ = write!(output, "{num}");
        output
    });

    for _ in 0..k {
        out = digits.chars().filter_map(|d| d.to_digit(10)).sum();
        digits = format!("{out}");
    }

    out as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = alphabet_index_digit_sum("abc", 1);
        assert_eq!(result, 6);
    }

    #[test]
    fn test2() {
        let result = alphabet_index_digit_sum("az", 2);
        assert_eq!(result, 9);
    }

    #[test]
    fn test3() {
        let result = alphabet_index_digit_sum("cat", 1);
        assert_eq!(result, 6);
    }

    #[test]
    fn test4() {
        let result = alphabet_index_digit_sum("dog", 2);
        assert_eq!(result, 8);
    }

    #[test]
    fn test5() {
        let result = alphabet_index_digit_sum("perl", 3);
        assert_eq!(result, 6);
    }
}
