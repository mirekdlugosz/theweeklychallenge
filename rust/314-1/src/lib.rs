pub fn equal_strings(s1: &str, s2: &str, s3: &str) -> isize {
    let mut shortest = s1;
    let mut remaining = vec![s2, s3];
    let mut removed_count = 0;

    if shortest.len() > s2.len() {
        shortest = s2;
        remaining = vec![s1, s3];
    }
    if shortest.len() > s3.len() {
        shortest = s3;
        remaining = vec![s1, s2];
    }

    while ! shortest.is_empty() {
        let found = remaining
            .iter()
            .all(|s| s.starts_with(shortest));

        if found {
            let sum: usize = remaining
                .iter()
                .map(|s| s.chars().count() - shortest.chars().count())
                .sum();
            return sum as isize + removed_count;
        }

        let mut new_shortest = shortest.chars();
        new_shortest.next_back();
        shortest = new_shortest.as_str();
        removed_count += 1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = equal_strings("abc", "abb", "ab");
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = equal_strings("ayz", "cyz", "xyz");
        assert_eq!(result, -1);
    }

    #[test]
    fn test3() {
        let result = equal_strings("yza", "yzb", "yzc");
        assert_eq!(result, 3);
    }
}
