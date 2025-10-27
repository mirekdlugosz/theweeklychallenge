pub fn last_visitor(ints: &[isize]) -> Vec<isize> {
    let mut ans = Vec::with_capacity(ints.len());
    let mut seen: Vec<isize> = Vec::with_capacity(ints.len());
    let mut minus_in_row = 0;
    for i in ints {
        if *i > 0 {
            minus_in_row = 0;
            seen.insert(0, *i);
        } else {
            // the exercise wants us to check $x < len(@seen), but that's the same as
            // fallible attempt at grabbing @seen[$x]
            if let Some(elem) = seen.get(minus_in_row) {
                ans.push(*elem);
            } else {
                ans.push(-1);
            }
            minus_in_row += 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = last_visitor(&[5, -1, -1]);
        assert_eq!(result, [5, -1]);
    }

    #[test]
    fn test2() {
        let result = last_visitor(&[3, 7, -1, -1, -1]);
        assert_eq!(result, [7, 3, -1]);
    }

    #[test]
    fn test3() {
        let result = last_visitor(&[2, -1, 4, -1, -1]);
        assert_eq!(result, [2, 4, 2]);
    }

    #[test]
    fn test4() {
        let result = last_visitor(&[10, 20, -1, 30, -1, -1]);
        assert_eq!(result, [20, 30, 20]);
    }

    #[test]
    fn test5() {
        let result = last_visitor(&[-1, -1, 5, -1]);
        assert_eq!(result, [-1, -1, 5]);
    }
}
