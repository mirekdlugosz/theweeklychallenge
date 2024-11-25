pub fn contiguous_array(binary: &[usize]) -> usize {
    binary.iter()
        .enumerate()
        .map(|(idx, _)| {
            // check the array from current index to end, and make it shorter on each
            // iteration. This way we don't waste time checking shorter arrays when
            // longer already meets criteria
            let mut last_idx = binary.len();
            while last_idx > idx {
                if let Some(current_array) = binary.get(idx..=last_idx) {
                    let current_array_len = current_array.len();
                    // arrays of uneven length can't have equal number of zeroes and ones
                    if current_array_len % 2 != 0 {
                        last_idx -= 1;
                        continue;
                    }

                    let (zeroes, ones): (Vec<usize>, Vec<usize>) = current_array
                        .iter()
                        .partition(|i| **i == 0);

                    if zeroes.len() == ones.len() {
                        return current_array.len();
                    }
                }
                last_idx -= 1;
            }
            0
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = contiguous_array(&[1, 0]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = contiguous_array(&[0, 1, 0]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let result = contiguous_array(&[0, 0, 0, 0, 0]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = contiguous_array(&[0, 1, 0, 0, 1, 0]);
        assert_eq!(result, 4);
    }
}
