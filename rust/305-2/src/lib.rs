use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter;

pub fn alien_dictionary<'a>(words: &[&'a str], alien: &[char]) -> Vec<&'a str> {
    let mut sorted_words = words.to_vec();

    let mut alien_lookup: HashMap<&char, isize> = alien
        .iter()
        .enumerate()
        .map(|(idx, ch)| (ch, idx.try_into().unwrap()))
        .collect();

    alien_lookup.insert(&'\0', -1);

    sorted_words.sort_by(|a, b| {
        let zipped: Box<dyn Iterator<Item = (char, char)>> = if a.len() > b.len() {
            Box::new(iter::zip(a.chars(), b.chars().chain(iter::repeat('\0'))))
        } else {
            Box::new(iter::zip(a.chars().chain(iter::repeat('\0')), b.chars()))
        };
        for (letter_a, letter_b) in zipped {
            let letter_a_idx = alien_lookup.get(&letter_a).unwrap_or(&-1);
            let letter_b_idx = alien_lookup.get(&letter_b).unwrap_or(&-1);

            match letter_a_idx.cmp(letter_b_idx) {
                Ordering::Equal => continue,
                ord => return ord,
            }
        }
        Ordering::Equal
    });
    sorted_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = alien_dictionary(
            &["perl", "python", "raku"],
            &[
                'h', 'l', 'a', 'b', 'y', 'd', 'e', 'f', 'g', 'i', 'r', 'k', 'm', 'n', 'o', 'p',
                'q', 'j', 's', 't', 'u', 'v', 'w', 'x', 'c', 'z',
            ],
        );
        let expected = vec!["raku", "python", "perl"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = alien_dictionary(
            &["the", "weekly", "challenge"],
            &[
                'c', 'o', 'r', 'l', 'd', 'a', 'b', 't', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm',
                'n', 'p', 'q', 's', 'w', 'u', 'v', 'x', 'y', 'z',
            ],
        );
        let expected = vec!["challenge", "the", "weekly"];
        assert_eq!(result, expected);
    }
}
