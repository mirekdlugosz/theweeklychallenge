pub fn semi_ordered_permutation(ints: &[usize]) -> usize {
    // We are tasked with telling how many adjacent swaps we have to make
    // to put 1 on far-left position and n on far-right position. We don't
    // need to make any swaps to learn that.
    //
    // The index of 1 in array tells us how many swaps we need to put it on
    // far-left position.
    // The same is true for n, except we need reversed array. Or we can
    // calculate the difference between last possible index and current index
    // of n. Note that arrays are 0-indexed, so last possible index is n - 1.
    //
    // If 1 is closer to far-right than n (meaning n is closer to far-left than 1),
    // then at one point we will have to swap 1 with n. We need to subtract 1 from
    // total number of swaps calculated with above formula to avoid counting that
    // swap twice.
    //
    // Since task says "You are given permutation of $n integers", we assume
    // this is valid permutation and don't bother checking if 1, n and 
    // everything in-between is actually present in ints.
    let last = ints.len();
    
    let first_idx = ints.iter().position(|&n| n == 1).unwrap();
    let last_idx = ints.iter().position(|&n| n == last).unwrap();

    let swaps = first_idx + (last - 1 - last_idx);
    match first_idx > last_idx {
        true => swaps - 1, // we swapped 1 and n at some point
        false => swaps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = semi_ordered_permutation(&[2, 1, 4, 3]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = semi_ordered_permutation(&[2, 4, 1, 3]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let result = semi_ordered_permutation(&[1, 3, 2, 4, 5]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = semi_ordered_permutation(&[5, 4, 3, 2, 1]);
        assert_eq!(result, 7);
    }
}
