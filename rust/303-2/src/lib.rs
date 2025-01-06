use std::collections::HashMap;

pub fn delete_and_earn(ints: &[usize]) -> usize {
    let mut total: usize = 0;
    let mut ints: Vec<_> = ints.into();
    while !ints.is_empty() {
        let mut elems: HashMap<&usize, usize> = HashMap::new();
        for elem in &ints {
            elems.entry(elem)
                .and_modify(|e| *e += elem)
                .or_insert(*elem);
        }

        let (largest_elem, largest_sum) = elems.iter()
            .filter(|kv| {
                let mut surrounding_sum = 0;
                let key = **kv.0;
                if let Some(e) = elems.get_key_value(&(key - 1)) {
                    surrounding_sum += e.1;
                }
                if let Some(e) = elems.get_key_value(&(key + 1)) {
                    surrounding_sum += e.1;
                }
                *kv.1 >= surrounding_sum
            })
            .max_by(|a, b| a.1.cmp(b.1))
            .map_or((0, 0), |kv| (**kv.0, *kv.1));

        if largest_sum == 0 {
            break;
        }

        total += largest_elem;

        if let Some(elem_idx) = ints.iter().position(|&x| x == largest_elem) {
            ints.swap_remove(elem_idx);
            ints.retain(|&x| x != largest_elem - 1 && x != largest_elem + 1);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = delete_and_earn(&[3, 4, 2]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test2() {
        let result = delete_and_earn(&[2, 2, 3, 3, 3, 4]);
        assert_eq!(result, 9);
    }

    #[test]
    fn test3() {
        let result = delete_and_earn(&[2, 3, 4, 5]);
        assert_eq!(result, 8);
    }

    #[test]
    fn test4() {
        let result = delete_and_earn(&[2, 3, 4, 4, 5]);
        assert_eq!(result, 10);
    }

    #[test]
    fn test5() {
        let result = delete_and_earn(&[2, 3, 4, 4, 6]);
        assert_eq!(result, 16);
    }

    #[test]
    fn test6() {
        let result = delete_and_earn(&[2, 2, 2, 3, 3, 4]);
        assert_eq!(result, 10);
    }
}
