use std::collections::HashSet;

fn set_count(ints: &[usize], mut idx: usize) -> usize {
    let mut seen: HashSet<usize> = HashSet::new();

    loop {
        seen.insert(idx);
        let next_idx = ints.get(idx).unwrap_or(&idx);
        if seen.contains(next_idx) {
            break;
        }
        idx = *next_idx;
    }

    seen.len()
}

pub fn nested_array(ints: &[usize]) -> usize {
    ints.iter()
        .enumerate()
        .map(|(i, _)| set_count(ints, i))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = nested_array(&[5, 4, 0, 3, 1, 6, 2]);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let result = nested_array(&[0, 1, 2]);
        assert_eq!(result, 1);
    }
}
