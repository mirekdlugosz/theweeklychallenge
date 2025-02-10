use std::collections::HashSet;

pub fn count_common(str1: &[&str], str2: &[&str]) -> usize {
    // this won't work if either argument has duplicate elements,
    // and definitely won't work if both have the same element duplicated
    // but it's unclear what should happen in such case
    let hash1: HashSet<&&str> = str1.iter().collect();
    let hash2: HashSet<&&str> = str2.iter().collect();
    hash1.intersection(&hash2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = count_common(
            &["perl", "weekly", "challenge"],
            &["raku", "weekly", "challenge"],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = count_common(
            &["perl", "raku", "python"],
            &["python", "java"],
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let result = count_common(
            &["guest", "contribution"],
            &["fun", "weekly", "challenge"],
        );
        assert_eq!(result, 0);
    }
}
