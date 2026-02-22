type Matrix = Vec<Vec<usize>>;

pub fn find_celebrity(matrix: &Matrix) -> Option<usize> {
    let mut celebrities: Vec<usize> = Vec::new();

    for (idx, row) in matrix.iter().enumerate() {
        if row.iter().sum::<usize>() == 0 {
            celebrities.push(idx);
        }
    }

    if celebrities.is_empty() {
        return None;
    }

    if celebrities.len() > 1 {
        return None;
    }

    let celebrity = *(celebrities.first().unwrap());

    matrix
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if idx == celebrity {
                None
            } else {
                row.get(celebrity).map(|match_| *match_ == 1)
            }
        })
        .all(|r| r)
        .then_some(celebrity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let matrix = vec![
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
        ];
        let result = find_celebrity(&matrix);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test2() {
        let matrix = vec![
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 0, 0],
        ];
        let result = find_celebrity(&matrix);
        assert_eq!(result, None);
    }

    #[test]
    fn test3() {
        let matrix = vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
        ];
        let result = find_celebrity(&matrix);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test4() {
        let matrix = vec![
            vec![0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![1, 0, 1, 1, 0, 0],
        ];
        let result = find_celebrity(&matrix);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test5() {
        let matrix = vec![
            vec![0, 1, 1, 0],
            vec![1, 0, 1, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let result = find_celebrity(&matrix);
        assert_eq!(result, None);
    }

    #[test]
    fn test6() {
        let matrix = vec![
            vec![0, 0, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 1, 0, 1],
            vec![1, 1, 0, 0],
        ];
        let result = find_celebrity(&matrix);
        assert_eq!(result, None);
    }
}
