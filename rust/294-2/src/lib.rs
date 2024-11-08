pub fn next_permutation(ints: &[isize]) -> Option<Vec<isize>> {
    let k = ints.iter()
        .enumerate()
        .filter_map(|(idx, i)| {
            let next_idx = idx + 1;
            let j = ints.get(next_idx)?;
            match i < j {
                true => Some(idx),
                false => None,
            }
        })
        .last()?;

    let l = ints.iter()
        .enumerate()
        .filter_map(|(idx, i)| {
            match ints.get(k)? < i {
                true => Some(idx),
                false => None,
            }
        })
        .last()?;

    if k > l {
        return None;
    }

    let mut swapped: Vec<isize> = ints.into();
    swapped.swap(k, l);

    let mut new_permutation: Vec<isize> = Vec::new();
    for i in swapped[..=k].iter() {
        new_permutation.push(*i);
    }
    for i in swapped[(k + 1)..].iter().rev() {
        new_permutation.push(*i);
    }

    Some(new_permutation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = next_permutation(&[1, 2, 3]);
        assert_eq!(result, Some(vec![1, 3, 2]));
    }

    #[test]
    fn test2() {
        let result = next_permutation(&[2, 1, 3]);
        assert_eq!(result, Some(vec![2, 3, 1]));
    }

    #[test]
    fn test3() {
        let result = next_permutation(&[3, 1, 2]);
        assert_eq!(result, Some(vec![3, 2, 1]));
    }

    #[test]
    fn test4() {
        let result = next_permutation(&[2, 3, 1]);
        assert_eq!(result, Some(vec![3, 1, 2]));
    }

    #[test]
    fn test5() {
        let result = next_permutation(&[3, 2, 1]);
        assert_eq!(result, None);
    }
}
