static LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn encrypted_string(str: &str, int: usize) -> String {
    let offset = int.rem_euclid(LETTERS.len());
    let ascii_offset = 'a' as u8;
    if offset == 0 {
        return str.to_string();
    }
    let mut output = String::new();
    for ch in str.chars() {
        let orig = ch as u8;
        let new = offset.saturating_add(orig as usize);
        let new = ((new as u8) - ascii_offset).rem_euclid(LETTERS.len() as u8);
        if let Some(new) = LETTERS.get(new as usize) {
            output.push(*new);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = encrypted_string("abc", 1);
        assert_eq!(result, "bcd");
    }

    #[test]
    fn test2() {
        let result = encrypted_string("xyz", 2);
        assert_eq!(result, "zab");
    }

    #[test]
    fn test3() {
        let result = encrypted_string("abc", 27);
        assert_eq!(result, "bcd");
    }

    #[test]
    fn test4() {
        let result = encrypted_string("hello", 5);
        assert_eq!(result, "mjqqt");
    }

    #[test]
    fn test5() {
        let result = encrypted_string("perl", 26);
        assert_eq!(result, "perl");
    }
}
