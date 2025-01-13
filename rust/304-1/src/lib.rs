pub fn arrange_binary(digits: &[u8], n: usize) -> bool {
    // The basic idea is:
    // 1. Extract chunks of consecutive 0
    // 2. For each chunk, how many 0 can we safely flip to 1?
    // 3. Sum that for all the chunks
    // 4. Is the sum at least n?
    //
    // The core question is: given n 0s, how many of them can I safely flip?
    // The answer turns out to be floor((n - 1) / 2). That return 0 for {1, 2},
    // then 1 for {3, 4}, 2 for {5, 6} etc.
    // But wait!
    // If the chunk is at the beginning or end of `digits`, then the formula is
    // floor(n / 2). Because it's 0 for {1}, but 1 for {2, 3}, 2 for {4, 5} etc.
    // To know if first chunk is 0, we special-case first iteration of the loop
    // and check if `digits` begins with 0.
    // To know if last chunk is 0, we .peek() into iterator.
    let starts_with_zero = digits.first().map_or(false, |&x| x == 0);
    let mut seen_first = false;

    let mut max_n: usize = 0;

    let mut zeroes = digits
        .split(|&x| x == 1)
        .peekable();

    while let Some(chunk) = zeroes.next() {
        // split may produce chunks of zero-length. We can't filter them out
        // when constructing iterator, because then we wouldn't be able to
        // differentiate {..., 0, 0, 1} from {..., 0, 0}.
        if chunk.is_empty() {
            continue;
        }
        let first_chunk = starts_with_zero && !seen_first;
        let last_chunk = zeroes.peek().is_none();

        let correction = usize::from(!(first_chunk || last_chunk));

        let chunk_n = (chunk.len() - correction).div_euclid(2);
        max_n += chunk_n;

        // process entire input only in the worst case
        if max_n >= n {
            return true;
        }

        if !seen_first {
            seen_first = true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = arrange_binary(&[1, 0, 0, 0, 1], 1);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = arrange_binary(&[1, 0, 0, 0, 1], 2);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = arrange_binary(&[1, 0, 0, 0, 1, 0, 0, 0], 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let result = arrange_binary(&[0, 0, 1, 0, 0, 1], 1);
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = arrange_binary(&[1, 0, 0, 0, 1, 0, 0, 0, 1], 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test6() {
        let result = arrange_binary(&[1, 0, 0, 0, 0, 0, 1], 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test7() {
        let result = arrange_binary(&[1, 1, 1, 0, 0, 1], 1);
        assert_eq!(result, false);
    }
}
