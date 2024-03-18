pub fn multiply(ints: &[usize], start: usize) -> usize {
    let mut current = start;
    while let true = ints.contains(&current) {
        current *= 2;
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ints = vec![5,3,6,1,12];
        assert_eq!(multiply(&ints, 3), 24);
    }

    #[test]
    fn test2() {
        let ints = vec![1,2,4,3];
        assert_eq!(multiply(&ints, 1), 8);
    }

    #[test]
    fn test3() {
        let ints = vec![5,6,7];
        assert_eq!(multiply(&ints, 2), 2);
    }
}
