pub fn decompressed_list(ints: &[usize]) -> Vec<usize> {
    let mut output = Vec::with_capacity(ints.len());
    for chunk in ints.chunks_exact(2) {
        let times = *chunk.first().unwrap_or(&0);
        let item = *chunk.last().unwrap_or(&0);

        for _ in 0..times {
            output.push(item);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = decompressed_list(&[1, 3, 2, 4]);
        assert_eq!(result, vec!(3, 4, 4));
    }

    #[test]
    fn test2() {
        let result = decompressed_list(&[1, 1, 2, 2]);
        assert_eq!(result, vec!(1, 2, 2));
    }

    #[test]
    fn test3() {
        let result = decompressed_list(&[3, 1, 3, 2]);
        assert_eq!(result, vec!(1, 1, 1, 2, 2, 2));
    }
}
