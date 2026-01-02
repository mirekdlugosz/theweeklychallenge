type Matrix = Vec<Vec<usize>>;

// The rules and examples are daunting, but this task is conceptually very simple.
// Think of matrix as "wrapped vector" - that is, matrix is just a vector of values,
// and rows exist only on display (AFAIK this is how matrices work in R).
// In that model, the entire task boils down to "take k values from the end and put them
// at the beginning".
// The only problem is that Rust does not have "take from right", so we have to
// use .skip() and .take() instead, and then we need to acknowledge that matrix
// IS NOT a "wrapper vector" - we have to turn it into a vector, and then turn it
// back into vector of vectors.
pub fn shift_grid(matrix: &Matrix, k: usize) -> Matrix {
    let mut output: Matrix = Vec::with_capacity(matrix.len());
    let row_size = matrix.first().map_or(0, Vec::len);
    let stream: Vec<_> = matrix.iter().flatten().collect();
    let offset = stream.len() - k;
    let shifted = stream.iter().skip(offset).chain(stream.iter().take(offset));
    let mut row: Vec<usize> = Vec::with_capacity(row_size);
    for value in shifted {
        row.push(**value);
        if row.len() == row_size {
            output.push(row.clone());
            row.clear();
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let result = shift_grid(&matrix, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let matrix = vec![vec![10, 20], vec![30, 40]];
        let expected = vec![vec![40, 10], vec![20, 30]];
        let result = shift_grid(&matrix, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let expected = vec![vec![6, 1], vec![2, 3], vec![4, 5]];
        let result = shift_grid(&matrix, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![2, 3, 4], vec![5, 6, 1]];
        let result = shift_grid(&matrix, 5);
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let matrix = vec![vec![1, 2, 3, 4]];
        let expected = vec![vec![4, 1, 2, 3]];
        let result = shift_grid(&matrix, 1);
        assert_eq!(result, expected);
    }
}
