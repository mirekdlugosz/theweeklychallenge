type Matrix = Vec<Vec<isize>>;

pub fn is_x_matrix(matrix: &Matrix) -> bool {
    let row_count = matrix.len() - 1; // we use that to calculate position in an array
                                      // and indexes are zero-based

    for (i, row) in matrix.iter().enumerate() {
        let non_zero_idx = vec![i, row_count - i];

        for (j, number) in row.iter().enumerate() {
            let is_zero = *number == 0;
            let should_be_zero = !non_zero_idx.contains(&j);

            if is_zero != should_be_zero {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let matrix = vec![
            vec![1, 0, 0, 2],
            vec![0, 3, 4, 0],
            vec![0, 5, 6, 0],
            vec![7, 0, 0, 1],
        ];
        assert_eq!(is_x_matrix(&matrix), true);
    }

    #[test]
    fn test2() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(is_x_matrix(&matrix), false);
    }

    #[test]
    fn test3() {
        let matrix = vec![
            vec![1, 0, 2],
            vec![0, 3, 0],
            vec![4, 0, 5],
        ];
        assert_eq!(is_x_matrix(&matrix), true);
    }
}
