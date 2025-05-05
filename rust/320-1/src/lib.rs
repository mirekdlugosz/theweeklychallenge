pub fn maximum_count(ints: &[isize]) -> usize {
    let (negative, positive): (Vec<isize>, Vec<isize>) =
        ints.iter().filter(|i| **i != 0).partition(|i| 0 > **i);
    negative.len().max(positive.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = maximum_count(&[-3, -2, -1, 1, 2, 3]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = maximum_count(&[-2, -1, 0, 0, 1]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let result = maximum_count(&[1, 2, 3, 4]);
        assert_eq!(result, 4);
    }
}
