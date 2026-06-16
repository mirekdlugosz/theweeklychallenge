use std::collections::HashMap;

fn word_as_num(str: &str, map: &HashMap<&char, char>) -> usize {
    str.chars()
        .filter_map(|c| map.get(&c))
        .collect::<String>()
        .parse()
        .unwrap_or(0)
}

pub fn sum_of_words(str1: &str, str2: &str, str3: &str) -> bool {
    let map: HashMap<&char, char> = HashMap::from([
        (&'a', '0'),
        (&'b', '1'),
        (&'c', '2'),
        (&'d', '3'),
        (&'e', '4'),
        (&'f', '5'),
        (&'g', '6'),
        (&'h', '7'),
        (&'i', '8'),
        (&'j', '9'),
    ]);

    let num1 = word_as_num(str1, &map);
    let num2 = word_as_num(str2, &map);
    let num3 = word_as_num(str3, &map);

    num1.wrapping_add(num2) == num3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = sum_of_words("acb", "cba", "cdb");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = sum_of_words("aab", "aac", "ad");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = sum_of_words("bc", "je", "jg");
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = sum_of_words("a", "aaaa", "a");
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = sum_of_words("c", "d", "h");
        assert_eq!(result, false);
    }

    #[test]
    fn test6() {
        let result = sum_of_words("gfi", "hbf", "bdhd");
        assert_eq!(result, true);
    }
}
