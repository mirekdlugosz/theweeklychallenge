pub fn consecutive_one(ints: &[usize]) -> usize {
    ints.chunk_by(|a, b| a == b)
        .filter_map(|chunk| {
            if chunk.first().unwrap_or(&0) == &0 {
                None
            } else {
                Some(chunk.len())
            }
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = consecutive_one(&[0, 1, 1, 0, 1, 1, 1]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = consecutive_one(&[0, 0, 0, 0]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let result = consecutive_one(&[1, 0, 1, 0, 1, 1]);
        assert_eq!(result, 2);
    }
}
