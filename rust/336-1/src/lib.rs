use std::collections::HashMap;

pub fn equal_group(ints: &[isize]) -> bool {
    let mut map: HashMap<isize, usize> = HashMap::with_capacity(ints.len());
    for num in ints {
        map.entry(*num).and_modify(|n| *n += 1).or_insert(1);
    }
    let mut max_value = *(map.values().max().unwrap_or(&0));
    let min_value = *(map.values().min().unwrap_or(&0));
    if 2 > min_value {
        return false;
    }

    // there's probably some clever trick to skip some numbers, or tell
    // there would be no grouping based on numbers alone
    while max_value > 1 {
        if map.values().all(|n| n % max_value == 0) {
            return true;
        }
        max_value -= 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = equal_group(&[1, 1, 2, 2, 2, 2]);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = equal_group(&[1, 1, 1, 2, 2, 2, 3, 3]);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = equal_group(&[5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7]);
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let result = equal_group(&[1, 2, 3, 4]);
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let result = equal_group(&[8, 8, 9, 9, 10, 10, 11, 11]);
        assert_eq!(result, true);
    }
}
