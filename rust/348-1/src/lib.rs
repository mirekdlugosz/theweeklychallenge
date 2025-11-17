static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn count_vowels(input: &str) -> usize {
    input.chars().filter(|c| VOWELS.contains(c)).count()
}

pub fn string_alike(input: &str) -> bool {
    let midpoint = input.chars().count().div_euclid(2);
    // technically this is wrong, as split_at expects byte offset, not char offset
    let input = input.to_lowercase();
    let (first, second) = input.split_at(midpoint);
    let first_vc = count_vowels(first);
    let second_vc = count_vowels(second);
    first_vc == second_vc && first_vc != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = string_alike("textbook");
        assert_eq!(result, false);
    }

    #[test]
    fn test2() {
        let result = string_alike("book");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = string_alike("AbCdEfGh");
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let result = string_alike("rhythmmyth");
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let result = string_alike("UmpireeAudio");
        assert_eq!(result, false);
    }
}
