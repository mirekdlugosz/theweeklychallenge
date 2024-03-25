pub fn count_eq_div(ints: &[usize], k: usize) -> usize {
    let mut count = 0;

    for (idx, num) in ints.iter().enumerate() {
        let nidx = idx + 1;  // next index, so we don't compare number to itself
        for (tjdx, anum) in ints[nidx..].iter().enumerate() {
            let jdx = idx + tjdx;  // second enumerate counts from 0, so we need
                                   // to calculate real index of second number

            let equal = num == anum;
            let divisible = (idx * jdx) % k == 0;

            if equal && divisible {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ints = vec![3,1,2,2,2,1,3];
        assert_eq!(count_eq_div(&ints, 2), 4);
    }

    #[test]
    fn test2() {
        let ints = vec![1,2,3];
        assert_eq!(count_eq_div(&ints, 1), 0);
    }
}
