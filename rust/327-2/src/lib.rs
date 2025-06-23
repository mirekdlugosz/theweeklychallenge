use std::cmp::Ordering;

pub fn mad(ints: &[isize]) -> Vec<(isize, isize)> {
    let mut lowest_mad = usize::MAX;
    let mut found: Vec<(isize, isize)> = Vec::with_capacity(ints.len());

    let mut sorted: Vec<isize> = ints.to_vec();
    sorted.sort_unstable();

    for chunk in sorted.windows(2) {
        let a = chunk.first().unwrap_or(&isize::MIN);
        let b = chunk.last().unwrap_or(&isize::MAX);

        let diff = a.abs_diff(*b);

        match diff.cmp(&lowest_mad) {
            Ordering::Less => {
                lowest_mad = diff;
                found.clear();
                found.push((*a, *b));
            }
            Ordering::Equal => found.push((*a, *b)),
            Ordering::Greater => (),
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = mad(&[4, 1, 2, 3]);
        assert_eq!(result, vec![(1, 2), (2, 3), (3, 4)]);
    }

    #[test]
    fn test2() {
        let result = mad(&[1, 3, 7, 11, 15]);
        assert_eq!(result, vec![(1, 3)]);
    }

    #[test]
    fn test3() {
        let result = mad(&[1, 5, 3, 8]);
        assert_eq!(result, vec![(1, 3), (3, 5)]);
    }
}
