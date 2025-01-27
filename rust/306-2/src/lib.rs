pub fn last_element(ints: &[isize]) -> isize {
    let mut board = ints.to_vec();

    while board.len() > 1 {
        board.sort_unstable();
        
        let y = board.swap_remove(board.len() - 1);
        let x = board.swap_remove(board.len() - 1);

        if y != x {
            board.push(y - x);
        }
    }

    *board.first().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = last_element(&[3, 8, 5, 2, 9, 2]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = last_element(&[3, 2, 5]);
        assert_eq!(result, 0);
    }
}
