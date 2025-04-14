pub fn friendly_strings(str1: &str, str2: &str) -> bool {
    if str1.chars().count() != str2.chars().count() {
        return false;
    }

    let mut differences: Vec<usize> = Vec::new();

    for (idx, (c1, c2)) in str1.chars().zip(str2.chars()).enumerate() {
        if c1 != c2 {
            differences.push(idx);
        }
    }

    if differences.len() != 2 {
        return false;
    }

    let mut new_str1: Vec<char> = str1.chars().collect();
    new_str1.swap(*differences.first().unwrap(), *differences.last().unwrap());

    new_str1.into_iter().collect::<String>() == str2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = friendly_strings("desc", "dsec");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = friendly_strings("fuck", "fcuk");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = friendly_strings("poo", "eop");
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = friendly_strings("stripe", "sprite");
        assert_eq!(result, true);
    }
}
