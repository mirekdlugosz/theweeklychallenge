pub fn target_array(source: &[isize], indices: &[usize]) -> Vec<isize> {
    let mut output: Vec<isize> = Vec::with_capacity(indices.len());

    for (i, idx) in indices.iter().enumerate() {
        let value = source[i];
        output.insert(*idx, value);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let source = vec!(0, 1, 2, 3, 4);
        let indices = vec!(0, 1, 2, 2, 1);
        let result = vec!(0, 4, 1, 3, 2);
        assert_eq!(target_array(&source, &indices), result);
    }

    #[test]
    fn test2() {
        let source = vec!(1, 2, 3, 4, 0);
        let indices = vec!(0, 1, 2, 3, 0);
        let result = vec!(0, 1, 2, 3, 4);
        assert_eq!(target_array(&source, &indices), result);
    }

    #[test]
    fn test3() {
        let source = vec!(1);
        let indices = vec!(0);
        let result = vec!(1);
        assert_eq!(target_array(&source, &indices), result);
    }
}
