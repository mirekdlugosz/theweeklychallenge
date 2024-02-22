type Matrix = Vec<Vec<isize>>;

enum MatrixProblems {
    LeadingNonOne,
    ZeroRowsInMiddle,
    MisalignedLeadingOnes,
    NonZeroesInLeadingColumn,
}

//1. If a row does not consist entirely of zeros, then the first
//   nonzero number in the row is a 1. We call this the leading 1.
fn check_leading_one(matrix: &Matrix) -> Result<(), MatrixProblems> {
    matrix.iter()
        .try_for_each(|row| {
            let first = row.iter()
                .filter(|c| **c > 0)
                .next();
            if let Some(num) = first {
                if *num > 1 {
                    return Err(MatrixProblems::LeadingNonOne);
                }
            }
            Ok(())
        })
}

//2. If there are any rows that consist entirely of zeros, then
//   they are grouped together at the bottom of the matrix.
fn check_zero_rows_at_end(matrix: &Matrix) -> Result<(), MatrixProblems> {
    let m = matrix.iter()
        .map(|row| {  // true if all elements are zero
            row.iter()
                .filter(|c| **c > 0)
                .count() == 0
        })
        .skip_while(|row| *row == false)  // remove non-zeroed rows from top
        .skip_while(|row| *row == true)   // remove all-zeroed rows from bottom
        .count();
    // if any rows are still left, it means that non-zeroed row appears after all-zeroed row
    if m > 0 {
        return Err(MatrixProblems::ZeroRowsInMiddle);
    }
    Ok(())
}

//3. In any two successive rows that do not consist entirely of zeros,
//   the leading 1 in the lower row occurs farther to the right than
//   the leading 1 in the higher row.
fn check_leading_ones_alignment(matrix: &Matrix) -> Result<(), MatrixProblems> {
    let mut indexes = matrix.iter()
        .filter_map(|row| {
            row.iter()
                .position(|e| *e > 0)
        })
        .peekable();

    loop {
        let item = indexes.next().unwrap();
        let next_item = match indexes.peek() {
            Some(i) => *i,
            None => break,
        };
        if item > next_item {
            return Err(MatrixProblems::MisalignedLeadingOnes);
        }
    }
    Ok(())
}

//4. Each column that contains a leading 1 has zeros everywhere else
//   in that column.
fn check_zeroes_in_leading_columns(matrix: &Matrix) -> Result<(), MatrixProblems> {
    let indexes = matrix.iter()
        .filter_map(|row| {
            row.iter()
                .position(|e| *e == 1)
        });
    for idx in indexes {
        let has_non_zeroes = matrix.iter()
            .filter_map(|row| row.get(idx))
            .filter(|e| **e > 0)
            .count();
        if has_non_zeroes > 1 {
            return Err(MatrixProblems::NonZeroesInLeadingColumn);
        }
    }
    Ok(())
}

fn check_rref(matrix: &Matrix) -> Result<(), MatrixProblems> {
    check_leading_one(matrix)?;
    check_zero_rows_at_end(matrix)?;
    check_leading_ones_alignment(matrix)?;
    check_zeroes_in_leading_columns(matrix)?;
    Ok(())
}

pub fn is_rref(matrix: &Matrix) -> bool {
    check_rref(&matrix).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = vec![
            vec![1, 1, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(is_rref(&matrix), false);
    }

    #[test]
    fn case2() {
        let matrix = vec![
            vec![0, 1,-2, 0, 1],
            vec![0, 0, 0, 1, 3],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0]
        ];
        assert_eq!(is_rref(&matrix), true);
    }

    #[test]
    fn case3() {
        let matrix = vec![
            vec![1, 0, 0, 4],
            vec![0, 1, 0, 7],
            vec![0, 0, 1,-1],
        ];
        assert_eq!(is_rref(&matrix), true);
    }

    #[test]
    fn case4() {
        let matrix = vec![
            vec![0, 1,-2, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 3],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(is_rref(&matrix), false);
    }

    #[test]
    fn case5() {
        let matrix = vec![
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(is_rref(&matrix), false);
    }

    #[test]
    fn case6() {
        let matrix = vec![
            vec![4, 0, 0, 0],
            vec![0, 1, 0, 7],
            vec![0, 0, 1,-1],
        ];
        assert_eq!(is_rref(&matrix), false);
    }
}
