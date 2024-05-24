type Matrix = Vec<Vec<usize>>;


fn is_special_position(matrix: &Matrix, i: usize, j: usize) -> bool {
    let row = &matrix[i];
    let column = matrix.iter().map(|row| row[j]);

    let row_numbers = &row.iter().filter(|x| **x != 0).count();
    let column_numbers = &column.filter(|x| *x != 0).count();

    *row_numbers == 1 && *column_numbers == 1
}

pub fn special_positions(matrix: &Matrix) -> usize {
    let mut special_positions_num: usize = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == 0 {
                continue;
            }
            match is_special_position(matrix, i, j) {
                true => {
                    special_positions_num += 1;
                    println!("{} {}", i, j);
                },
                false => (),
            }
        }
    }
    special_positions_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let matrix = vec![
            vec![1, 0, 0],
            vec![0, 0, 1],
            vec![1, 0, 0],
        ];
        assert_eq!(special_positions(&matrix), 1);
    }

    #[test]
    fn test2() {
        let matrix = vec![
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1],
        ];
        assert_eq!(special_positions(&matrix), 3);
    }

    #[test]
    fn test3() {
        let matrix = vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 1],
        ];
        assert_eq!(special_positions(&matrix), 2);
    }

    #[test]
    fn test4() {
        let matrix = vec![
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
        ];
        assert_eq!(special_positions(&matrix), 2);
    }
}
