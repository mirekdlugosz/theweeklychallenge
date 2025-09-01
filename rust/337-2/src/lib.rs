use std::iter::repeat_n;

pub fn odd_matrix(row: usize, col: usize, locations: &[(usize, usize)]) -> usize {
    let mut rows: Vec<usize> = repeat_n(0, row).collect();
    let mut cols: Vec<usize> = repeat_n(0, col).collect();
    for (row_loc, col_loc) in locations {
        if let Some(row_elem) = rows.get_mut(*row_loc) {
            *row_elem += 1;
        }
        if let Some(col_elem) = cols.get_mut(*col_loc) {
            *col_elem += 1;
        }
    }
    let mut total_odd = 0;
    for row_count in rows {
        for col_count in &cols {
            let is_odd = (row_count + col_count).rem_euclid(2) == 1;
            if is_odd {
                total_odd += 1;
            }
        }
    }
    total_odd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = odd_matrix(2, 3, &[(0, 1), (1, 1)]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test2() {
        let result = odd_matrix(2, 2, &[(1, 1), (1, 1)]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let result = odd_matrix(3, 3, &[(0, 0), (1, 2), (2, 1)]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = odd_matrix(1, 5, &[(0, 2), (0, 4)]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test5() {
        let result = odd_matrix(4, 2, &[(1, 0), (3, 1), (2, 0), (0, 1)]);
        assert_eq!(result, 8);
    }
}
