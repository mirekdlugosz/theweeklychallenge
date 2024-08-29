use std::collections::HashMap;

pub fn lucky_int(ints: &[isize]) -> isize {
    let mut counter: HashMap<&isize, isize> = HashMap::new();

    for int in ints {
        counter.entry(int)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    counter.iter()
        .filter_map(|(k, v)| {
            match *k == v {
                true => Some(**k),
                false => None
            }
        })
        .max()
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = lucky_int(&[2, 2, 3, 4]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = lucky_int(&[1, 2, 2, 3, 3, 3]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let result = lucky_int(&[1, 1, 1, 3]);
        assert_eq!(result, -1);
    }
}
