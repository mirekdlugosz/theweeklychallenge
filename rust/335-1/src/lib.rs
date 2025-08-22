use std::collections::HashMap;

// The naive approach is to loop through each letter in first word and check if that
// letter appears in all other words. If it does, keep it in output.
// However, if first word is "hello", and second word is "lock", the naive approach
// would add both "l". So we would need to not only check if letter exists in other
// word, but also remove that letter from other word. So first "l" would count, but
// then "lock" becomes "ock" and second "l" is not counted.
//
// Here, I took an approach which requires looping over each word only once.
// First, find the shortest word in the input.
// Second, turn that word into a map of letters and their counts.
// Then, we loop over remaining words.
// First, create a copy of map, with all keys set to 0.
// Second, for each letter of the word, increase count in copied map. Note
// that we don't count letters that appear in second word but do not appear
// in first.
// Third, modify the initial map by changing the value to lower of two counts.
// Fourth, remove all the items that are zero now. This makes each subsequent iteration
// do a little less work than the previous.
// Finally, exit early if map became empty.

pub fn common_characters(words: &[&str]) -> HashMap<char, usize> {
    let mut shortest = usize::MAX;
    let mut shortest_idx: usize = 0;
    for (idx, word) in words.iter().enumerate() {
        let len = word.chars().count();
        if shortest > len {
            shortest_idx = idx;
            shortest = len;
        }
    }
    let word = words.get(shortest_idx).unwrap();
    let mut result: HashMap<char, usize> = HashMap::with_capacity(word.chars().count());

    for letter in word.chars() {
        result.entry(letter).and_modify(|c| *c += 1).or_insert(1);
    }

    for (idx, other_word) in words.iter().enumerate() {
        if idx == shortest_idx {
            continue;
        }
        let mut other_hash: HashMap<char, usize> = result.keys().map(|e| (*e, 0)).collect();
        for letter in other_word.chars() {
            other_hash.entry(letter).and_modify(|c| *c += 1);
        }
        let mut zeroed: Vec<char> = Vec::with_capacity(result.keys().count());
        for (key, value) in &mut result {
            if let Some(other_value) = other_hash.get(key) {
                *value = (*value).min(*other_value);
                if value == &0 {
                    zeroed.push(*key);
                }
            }
        }
        for key in zeroed {
            result.remove(&key);
        }
        if result.is_empty() {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = common_characters(&["bella", "label", "roller"]);
        assert_eq!(result, HashMap::from([('e', 1), ('l', 2)]));
    }

    #[test]
    fn test2() {
        let result = common_characters(&["cool", "lock", "cook"]);
        assert_eq!(result, HashMap::from([('c', 1), ('o', 1)]));
    }

    #[test]
    fn test3() {
        let result = common_characters(&["hello", "world", "pole"]);
        assert_eq!(result, HashMap::from([('l', 1), ('o', 1)]));
    }

    #[test]
    fn test4() {
        let result = common_characters(&["abc", "def", "ghi"]);
        assert_eq!(result, HashMap::new());
    }

    #[test]
    fn test5() {
        let result = common_characters(&["aab", "aac", "aaa"]);
        assert_eq!(result, HashMap::from([('a', 2)]));
    }
}
