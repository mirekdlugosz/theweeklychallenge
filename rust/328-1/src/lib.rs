static LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn replace_all(str: &str) -> String {
    let mut new_str: Vec<char> = str.chars().collect();

    while let Some(replace_pos) = new_str.iter().position(|&c| c == '?') {
        let mut surrounding_chars = Vec::with_capacity(2);

        if let Some(prev_pos) = replace_pos.checked_sub(1) {
            let prev_char = new_str.get(prev_pos).unwrap_or(&'\0');
            surrounding_chars.push(*prev_char);
        }
        if let Some(next_pos) = replace_pos.checked_add(1) {
            let next_char = new_str.get(next_pos).unwrap_or(&'\0');
            surrounding_chars.push(*next_char);
        }

        for candidate in LETTERS {
            if surrounding_chars.contains(&candidate) {
                continue;
            }
            new_str[replace_pos] = candidate;
        }
    }

    new_str.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = replace_all("a?z");
        assert_ne!(result, "aaz");
        assert_ne!(result, "azz");
    }

    #[test]
    fn test2() {
        let result = replace_all("pe?k");
        assert_ne!(result, "peek");
        assert_ne!(result, "pekk");
    }

    #[test]
    fn test3() {
        let result = replace_all("gra?te");
        assert_ne!(result, "graate");
        assert_ne!(result, "gratte");
    }

    #[test]
    fn test4() {
        let result = replace_all("?ra?te");
        assert_ne!(result, "rraate");
        assert_ne!(result, "rratte");
    }

    #[test]
    fn test5() {
        let result = replace_all("g?a?te");
        assert_ne!(result, "ggaate");
        assert_ne!(result, "gaaate");
        assert_ne!(result, "ggatte");
        assert_ne!(result, "gaatte");
    }

    #[test]
    fn test6() {
        let result = replace_all("gr??te");
        assert_ne!(result, "grrrte");
        assert_ne!(result, "grrtte");
        assert_ne!(result, "grttte");
        assert_ne!(result.chars().nth(3), result.chars().nth(4));
    }
}
