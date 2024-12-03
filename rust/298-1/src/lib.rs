fn is_all_ones(matrix: &[&Vec<usize>], row_idx: usize, col_idx: usize, size: usize) -> bool {
    // this takes a square submatrix of matrix starting from position row_idx and col_idx
    // and checks if it has only 1
    for row in matrix.iter().skip(row_idx).take(size) {
        let has_zero = row.iter()
            .skip(col_idx)
            .take(size)
            .any(|&i| i == 0);

        if has_zero {
            return false;
        }
    }

    true
}


pub fn max_square(matrix: &[&Vec<usize>]) -> usize {
    let total_rows = matrix.len();
    let total_cols = matrix.first().map_or(0, |r| r.len());
    let mut max_square_so_far = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            // start with a largest square we can take from current position...
            let remaining_rows = total_rows - i;
            let remaining_cols = total_cols - j;
            let max_square_here = remaining_rows.min(remaining_cols);
            let mut current_square_size = max_square_here;

            // ... and make it smaller while we can, until we find square that is all 1
            while current_square_size > 0 {
                // we already found square larger than one we could make in this iteration
                // don't bother running anything - it won't change result anyway
                if max_square_so_far >= current_square_size {
                    current_square_size -= 1;
                    continue;
                }
                // this square is all 1! since we always try squares in decreasing size,
                // we are done with current position
                if is_all_ones(matrix, i, j, current_square_size) {
                    max_square_so_far = current_square_size;
                    break
                }
                current_square_size -= 1;
            }
        }
    }
    // we are expected to return AREA of square. Luckily these are squares, so we can keep side
    // length and double it after processing entire matrix
    max_square_so_far.pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let matrix = [
            &vec![1, 0, 1, 0, 0],
            &vec![1, 0, 1, 1, 1],
            &vec![1, 1, 1, 1, 1],
            &vec![1, 0, 0, 1, 0],
        ];
        let result = max_square(&matrix);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let matrix = [
            &vec![0, 1],
            &vec![1, 0],
        ];
        let result = max_square(&matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let matrix = [
            &vec![0],
        ];
        let result = max_square(&matrix);
        assert_eq!(result, 0);
    }
}
