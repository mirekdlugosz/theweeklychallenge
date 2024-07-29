pub fn sort_letters(letters: &[char], weights: &[usize]) -> String {
    let mut output_chars: Vec<char> = Vec::with_capacity(letters.len());
    let mut output: String = String::new();

    for _item in letters {
        output_chars.push('\0');
    }

    for (letter, weight) in letters.iter().zip(weights.iter()) {
        let index = weight - 1;
        output_chars[index] = *letter;
    }

    for letter in output_chars {
        output.push(letter);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = sort_letters(&['R', 'E', 'P', 'L'], &[3, 2, 1, 4]);
        assert_eq!(result, "PERL".to_string());
    }

    #[test]
    fn test2() {
        let result = sort_letters(&['A', 'U', 'R', 'K'], &[2, 4, 1, 3]);
        assert_eq!(result, "RAKU".to_string());
    }

    #[test]
    fn test3() {
        let result = sort_letters(&['O', 'H', 'Y', 'N', 'P', 'T'], &[5, 4, 2, 6, 1, 3]);
        assert_eq!(result, "PYTHON".to_string());
    }
}
