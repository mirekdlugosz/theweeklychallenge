use std::collections::HashSet;

static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

// The task itself is easy. Just take every possible continuous substring of length
// at least 5 and check if it contains only vowels, and all vowels.
// The most problematic part is that output can't be a set (see test3 - "aeiou" is both 
// at beginning and end of input, and we want both), but order of elements will depend
// on your substring creation algorithm. We would need something like UnorderedVector,
// where duplicates are allowed, but order of elements does not matter.
// Instead of that, we just changed the order of elements in tests to match what our
// implementation is doing.

fn is_vowel_string(str: &str) -> bool {
    if 5 > str.len() {
        return false;
    }
    let vowels: HashSet<char> = VOWELS.iter().copied().collect();
    let string: HashSet<char> = str.chars().collect();
    vowels == string
}

pub fn count_vowel(str: &str) -> Vec<String> {
    let mut output = Vec::new();
    let max_length = str.len();

    for start_idx in 0..=max_length {
        let current_length = max_length - start_idx;
        for offset in 0..=current_length {
            let end_idx = start_idx + offset;
            if let Some(sub) = str.get(start_idx..=end_idx)
                && is_vowel_string(sub)
            {
                output.push(sub.to_string());
            }
        }
    }

    output.sort_unstable_by_key(String::len);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = count_vowel("aeiou");
        let expected = vec!["aeiou"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = count_vowel("aaeeeiioouu");
        let expected = vec!["aeeeiioou", "aaeeeiioou", "aeeeiioouu", "aaeeeiioouu"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = count_vowel("aeiouuaxaeiou");
        let expected = vec!["aeiou", "aeiou", "aeiouu", "eiouua", "aeiouua"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let result = count_vowel("uaeiou");
        let expected = vec!["uaeio", "aeiou", "uaeiou"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let result = count_vowel("aeioaeioa");
        let expected: Vec<String> = Vec::new();
        assert_eq!(result, expected);
    }
}
