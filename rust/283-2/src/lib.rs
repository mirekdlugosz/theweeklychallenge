use std::collections::HashMap;

pub fn digit_count_value(ints: &[usize]) -> bool {
    let mut counter: HashMap<&usize, usize> = HashMap::new();

    for i in ints {
        counter.entry(i)
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
    }

    for (i, _) in ints.iter().enumerate() {
        let expected = ints.get(i).unwrap();
        let actual = counter.get(&i).unwrap_or(&0);
        if expected != actual {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = digit_count_value(&[1, 2, 1, 0]);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = digit_count_value(&[0, 3, 0]);
        assert_eq!(result, false);
    }
}
