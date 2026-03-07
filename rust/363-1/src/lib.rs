use regex::Regex;

static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

#[derive(PartialEq)]
struct Count {
    vowels: usize,
    consonants: usize,
}

fn count_vowels(input: &str) -> usize {
    input.chars().filter(|c| VOWELS.contains(c)).count()
}

fn parse_word(input: &str) -> usize {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

impl Count {
    fn from_string(str: &str) -> Self {
        let vowels = count_vowels(str);
        let consonants = str.chars().count() - vowels;
        Self { vowels, consonants }
    }

    fn from_re_match((vowels, consonants): (&str, &str)) -> Self {
        Self {
            vowels: parse_word(vowels),
            consonants: parse_word(consonants),
        }
    }
}

pub fn string_lie_detector(str: &str) -> bool {
    let re = Regex::new(r"([a-z]+) — ([a-z]+) vowels? and ([a-z]+) consonants?").unwrap();
    if let Some(matches) = re.captures(str) {
        let (_, [word, vowels, consonants]) = matches.extract();
        let count_string = Count::from_string(word);
        let count_description = Count::from_re_match((vowels, consonants));
        return count_string == count_description;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = string_lie_detector("aa — two vowels and zero consonants");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = string_lie_detector("iv — one vowel and one consonant");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = string_lie_detector("hello - three vowels and two consonants");
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = string_lie_detector("aeiou — five vowels and zero consonants");
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = string_lie_detector("aei — three vowels and zero consonants");
        assert_eq!(result, true);
    }
}
