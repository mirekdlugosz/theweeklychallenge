use std::collections::HashMap;

pub fn unique_number(ints: &[isize]) -> isize {
    let mut counter: HashMap<&isize, usize> = HashMap::new();

    for i in ints {
        counter.entry(i)
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
    }

    counter.iter()
        .find_map(|(key, value)| {
            match *value == 1 {
                true => Some(**key),
                false => None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = unique_number(&[3, 3, 1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = unique_number(&[3, 2, 4, 2, 4]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let result = unique_number(&[1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test4() {
        let result = unique_number(&[4, 3, 1, 1, 1, 4]);
        assert_eq!(result, 3);
    }
}
