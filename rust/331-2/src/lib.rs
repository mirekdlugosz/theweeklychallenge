use std::collections::HashMap;

pub fn buddy_strings(source: &str, target: &str) -> bool {
    // The obvious solution is to loop through all letters, swap it with all the subsequent letters
    // and check if result matches target.
    // But that's boring, so here we try something more interesting.
    // Instructions mention swapping a letter, singular. That means that words must differ by
    // exactly two characters - when you swap them, you get the other string.
    // However, when words are exactly the same, they might still meet the requirements.
    // "love" is false, because no matter which letters you swap, you can't get "love".
    // But "feed" is true, because if you swap "e" with other "e", you get back "feed".
    // So, when the words are the same, we need to know if there's a letter that appears at least
    // twice. If there is, we can swap any two instances of that letter around and we will get back
    // original word.

    if source.len() != target.len() {
        return false;
    }
    let different_letters = source
        .chars()
        .zip(target.chars())
        .filter(|(s_ch, t_ch)| s_ch != t_ch)
        .count();
    match different_letters {
        0 => {
            let mut map: HashMap<char, usize> = HashMap::with_capacity(source.len());
            for ch in source.chars() {
                map.entry(ch)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
            map.values().any(|&v| v > 1)
        }
        2 => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = buddy_strings("fuck", "fcuk");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = buddy_strings("love", "love");
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = buddy_strings("fodo", "food");
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let result = buddy_strings("feed", "feed");
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = buddy_strings("wow", "wow");
        assert_eq!(result, true);
    }

    #[test]
    fn test6() {
        let result = buddy_strings("see", "sea");
        assert_eq!(result, false);
    }

    #[test]
    fn test7() {
        let result = buddy_strings("time", "gone");
        assert_eq!(result, false);
    }

    #[test]
    fn test8() {
        let result = buddy_strings("cucumber", "customer");
        assert_eq!(result, false);
    }

    #[test]
    fn test9() {
        let result = buddy_strings("successes", "successes");
        assert_eq!(result, true);
    }

    #[test]
    fn test10() {
        let result = buddy_strings("deeded", "deeded");
        assert_eq!(result, true);
    }

    #[test]
    fn test11() {
        let result = buddy_strings("love", "food");
        assert_eq!(result, false);
    }

    #[test]
    fn test12() {
        let result = buddy_strings("listen", "silent");
        assert_eq!(result, false);
    }

    #[test]
    fn test13() {
        let result = buddy_strings("astronomer", "moonstarter");
        assert_eq!(result, false);
    }

    #[test]
    fn test14() {
        let result = buddy_strings("love", "lovely");
        assert_eq!(result, false);
    }
}
