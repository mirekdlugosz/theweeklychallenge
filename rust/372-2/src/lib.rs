use std::collections::HashMap;

pub fn largest_substring(str: &str) -> usize {
    let mut positions: HashMap<char, Vec<usize>> = HashMap::with_capacity(str.len());
    for (idx, ch) in str.chars().enumerate() {
        positions
            .entry(ch)
            .and_modify(|v| v.push(idx))
            .or_insert_with(|| vec![idx]);
    }
    positions
        .values()
        .filter_map(|v| {
            if 1 >= v.len() {
                None
            } else {
                let min = v.first().unwrap();
                let max = v.last().unwrap();
                // number of elements between two characters
                // (idx of last) - (idx of first) - 1
                Some(max.saturating_sub(*min).saturating_sub(1))
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = largest_substring("aaaaa");
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = largest_substring("abcdeba");
        assert_eq!(result, 5);
    }

    #[test]
    fn test3() {
        let result = largest_substring("abbc");
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = largest_substring("abcaacbc");
        assert_eq!(result, 4);
    }

    #[test]
    fn test5() {
        let result = largest_substring("laptop");
        assert_eq!(result, 2);
    }
}
