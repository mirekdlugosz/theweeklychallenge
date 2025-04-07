pub fn subsequence(str1: &str, str2: &str) -> bool {
    let mut letters = 0;
    let mut found = 0;
    let mut str2_iter = str2.chars();

    for letter in str1.chars() {
        letters += 1;
        for haystack in str2_iter.by_ref() {
            if letter == haystack {
                found += 1;
                break;
            }
        }
    }
    letters == found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = subsequence("uvw", "bcudvew");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = subsequence("aec", "abcde");
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = subsequence("sip", "javascript");
        assert_eq!(result, true);
    }
}
