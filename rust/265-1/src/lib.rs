use std::collections::HashMap;

pub fn third_appearance(ints: &[isize]) -> Option<isize> {
    let ints_num = ints.len();
    let mut counter: HashMap<&isize, usize> = HashMap::with_capacity(ints_num);

    for num in ints {
        counter
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    counter
        .iter()
        .filter_map(|(key, value)| {
            let percentage = *value as f64 / ints_num as f64;
            match percentage {
                x if x > 0.3_f64 => Some(*key),
                _ => None,
            }
        })
        .min_by(|x, y| x.cmp(y))
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let inputs = vec!(1,2,3,3,3,3,4,2);
        assert_eq!(third_appearance(&inputs), Some(3));
    }

    #[test]
    fn test2() {
        let inputs = vec!(1,1);
        assert_eq!(third_appearance(&inputs), Some(1));
    }

    #[test]
    fn test3() {
        let inputs = vec!(1,2,3);
        assert_eq!(third_appearance(&inputs), Some(1));
    }

    #[test]
    fn test4() {
        let inputs = vec!(1,2,3,4,5);
        assert_eq!(third_appearance(&inputs), None);
    }
}
