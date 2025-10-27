pub fn peak_positions(ints: &[isize]) -> Vec<usize> {
    let mut output = Vec::with_capacity(ints.len());
    for (idx, elem) in ints.iter().enumerate() {
        let (new_idx, overflow) = idx.overflowing_sub(1);
        let prev = if overflow {
            &isize::MIN
        } else {
            ints.get(new_idx).unwrap_or(&isize::MIN)
        };
        let (new_idx, overflow) = idx.overflowing_add(1);
        let next = if overflow {
            &isize::MIN
        } else {
            ints.get(new_idx).unwrap_or(&isize::MIN)
        };
        if elem > prev && elem > next {
            output.push(idx);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = peak_positions(&[1, 3, 2]);
        assert_eq!(result, [1]);
    }

    #[test]
    fn test2() {
        let result = peak_positions(&[2, 4, 6, 5, 3]);
        assert_eq!(result, [2]);
    }

    #[test]
    fn test3() {
        let result = peak_positions(&[1, 2, 3, 2, 4, 1]);
        assert_eq!(result, [2, 4]);
    }

    #[test]
    fn test4() {
        let result = peak_positions(&[5, 3, 1]);
        assert_eq!(result, [0]);
    }

    #[test]
    fn test5() {
        let result = peak_positions(&[1, 5, 1, 5, 1, 5, 1]);
        assert_eq!(result, [1, 3, 5]);
    }
}
