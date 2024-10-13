pub fn double_exist(ints: &[usize]) -> bool {
    for (i_idx, i) in ints.iter().enumerate() {
        for (j_idx, j) in ints.iter().enumerate() {
            if i_idx >= j_idx {
                continue;
            }
            if *i == 2 * *j {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = double_exist(&[6, 2, 3, 3]);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = double_exist(&[3, 1, 4, 13]);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = double_exist(&[2, 1, 4, 2]);
        assert_eq!(result, true);
    }
}
