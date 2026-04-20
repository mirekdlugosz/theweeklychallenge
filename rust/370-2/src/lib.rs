// I'm sure there's recursion-based solution out there, but I don't like this exercise
// enough to think about it

pub fn scramble_string(str1: &str, str2: &str) -> bool {
    // if we knew something about environment this function will operate in, there are two 
    // low-hanging fruits for early returns:
    // - strings aren't equal length
    // - str2 is not an anagram of str1

    for (idx, _) in str1.bytes().enumerate() {
        if let Some((p1, p2)) = str1.split_at_checked(idx) {
            for candidate in &[(p1, p2), (p2, p1)] {
                let p11 = candidate.0;
                let p22 = candidate.1;

                for (idx1, _) in p11.bytes().enumerate() {
                    if let Some((p111, p112)) = p11.split_at_checked(idx1) {
                        for (idx2, _) in p22.bytes().enumerate() {
                            if let Some((p221, p222)) = p22.split_at_checked(idx2) {
                                for final_candidate in &[
                                    format!("{p111}{p112}{p221}{p222}"),
                                    format!("{p111}{p112}{p222}{p221}"),
                                    format!("{p112}{p111}{p221}{p222}"),
                                    format!("{p112}{p111}{p222}{p221}"),
                                ] {
                                    if final_candidate == str2 {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = scramble_string("abc", "acb");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = scramble_string("abcd", "cdba");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = scramble_string("hello", "hiiii");
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = scramble_string("ateer", "eater");
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = scramble_string("abcd", "bdac");
        assert_eq!(result, false);
    }
}
