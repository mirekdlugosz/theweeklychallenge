use std::cmp::max;

pub fn max_pos_neg(ints: &[isize]) -> usize {
    let mut npos = 0;
    let mut nneg = 0;

    for num in ints {
        match *num {
            n if n > 0 => npos += 1,
            n if 0 > n => nneg += 1,
            _ => (),
        }
    }

    max(npos, nneg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ints = vec![-3, 1, 2, -1, 3, -2, 4];
        assert_eq!(max_pos_neg(&ints), 4);
    }

    #[test]
    fn test2() {
        let ints = vec![-1, -2, -3, 1];
        assert_eq!(max_pos_neg(&ints), 3);
    }

    #[test]
    fn test3() {
        let ints = vec![1, 2];
        assert_eq!(max_pos_neg(&ints), 2);
    }
}
