static LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn replace_digits(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut last_letter: char = '\0';

    for char in input.chars() {
        let new_char = match char.is_numeric() {
            true => {
                let start = LETTERS.iter().position(|ch| ch == &last_letter).unwrap();
                let offset = char.to_digit(10).unwrap() as usize;

                *LETTERS.get(start + offset).unwrap()
            },
            false => {
                last_letter = char;
                char
            },
        };
        output.push(new_char);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = replace_digits("a1c1e1");
        let expected = "abcdef".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = replace_digits("a1b2c3d4");
        let expected = "abbdcfdh".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = replace_digits("b2b");
        let expected = "bdb".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let result = replace_digits("a16z");
        let expected = "abgz".to_string();
        assert_eq!(result, expected);
    }
}
