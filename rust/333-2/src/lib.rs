use std::collections::VecDeque;

pub fn duplicate_zeros(ints: &[isize]) -> Vec<isize> {
    let mut queue: VecDeque<isize> = VecDeque::with_capacity(ints.len() * 2);
    for i in ints {
        queue.push_back(*i);
        if *i == 0 {
            queue.push_back(*i);
        }
    }
    queue.truncate(ints.len());
    queue.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = duplicate_zeros(&[1, 0, 2, 3, 0, 4, 5, 0]);
        assert_eq!(result, [1, 0, 0, 2, 3, 0, 0, 4]);
    }

    #[test]
    fn test2() {
        let result = duplicate_zeros(&[1, 2, 3]);
        assert_eq!(result, [1, 2, 3]);
    }

    #[test]
    fn test3() {
        let result = duplicate_zeros(&[1, 2, 3, 0]);
        assert_eq!(result, [1, 2, 3, 0]);
    }

    #[test]
    fn test4() {
        let result = duplicate_zeros(&[0, 0, 1, 2]);
        assert_eq!(result, [0, 0, 0, 0]);
    }

    #[test]
    fn test5() {
        let result = duplicate_zeros(&[1, 2, 0, 3, 4]);
        assert_eq!(result, [1, 2, 0, 0, 3]);
    }
}
