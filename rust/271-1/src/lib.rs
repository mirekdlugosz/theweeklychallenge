type Matrix = Vec<Vec<usize>>;

pub fn maximum_ones(matrix: &Matrix) -> usize {
    let counts = matrix
        .iter()
        .map(|row| row.iter().sum::<usize>());
    // there is a way to get position of max in single iteration,
    // but that would return *last* maximum, not the first one.
    // Alternatively, we could have use for loop ourselves
    let max = counts.clone().max().unwrap();
    let max_position = counts.clone().position(|c| c == max).unwrap();
    max_position + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let matrix = vec![
            vec![0, 1],
            vec![1, 0],
        ];
        assert_eq!(maximum_ones(&matrix), 1);
    }

    #[test]
    fn test2() {
        let matrix = vec![
            vec![0, 0, 0],
            vec![1, 0, 1],
        ];
        assert_eq!(maximum_ones(&matrix), 2);
    }

    #[test]
    fn test3() {
        let matrix = vec![
            vec![0, 0],
            vec![1, 1],
            vec![0, 0],
        ];
        assert_eq!(maximum_ones(&matrix), 2);
    }
}
