pub fn title_capital(str: &str) -> String {
    let new_str: Vec<String> = str
        .split(' ')
        .map(|word| {
            let word_len = word.chars().count();
            let mut new_word = String::with_capacity(word_len);
            let mut chars = word.chars();
            if word_len > 2 {
                if let Some(char) = chars.next() {
                    new_word.push(char.to_ascii_uppercase());
                }
            }
            for char in chars {
                new_word.push(char.to_ascii_lowercase());
            }
            new_word
        })
        .collect();
    new_str.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = title_capital("PERL IS gREAT");
        assert_eq!(result, "Perl is Great");
    }

    #[test]
    fn test2() {
        let result = title_capital("THE weekly challenge");
        assert_eq!(result, "The Weekly Challenge");
    }

    #[test]
    fn test3() {
        let result = title_capital("YoU ARE A stAR");
        assert_eq!(result, "You Are a Star");
    }
}
