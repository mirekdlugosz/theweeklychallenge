use std::collections::HashMap;

static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn sum_frequencies(str: &str) -> usize {
    let mut map: HashMap<char, usize> = HashMap::with_capacity(str.len());

    for ch in str.chars() {
        map.entry(ch).and_modify(|c| *c += 1).or_insert(1);
    }

    let (wovels, consonants): (Vec<_>, Vec<_>) =
        map.iter().partition(|(k, _v)| VOWELS.contains(*k));

    let max_wovel = wovels.iter().max_by_key(|(_, n)| n).map_or(0, |(_, n)| **n);
    let max_consonant = consonants
        .iter()
        .max_by_key(|(_, n)| n)
        .map_or(0, |(_, n)| **n);

    max_wovel + max_consonant
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = sum_frequencies("banana");
        assert_eq!(result, 5);
    }

    #[test]
    fn test2() {
        let result = sum_frequencies("teestett");
        assert_eq!(result, 7);
    }

    #[test]
    fn test3() {
        let result = sum_frequencies("aeiouuaa");
        assert_eq!(result, 3);
    }

    #[test]
    fn test4() {
        let result = sum_frequencies("rhythm");
        assert_eq!(result, 2);
    }

    #[test]
    fn test5() {
        let result = sum_frequencies("x");
        assert_eq!(result, 1);
    }
}
