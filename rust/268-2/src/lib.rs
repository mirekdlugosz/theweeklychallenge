pub fn number_game(input: &[isize]) -> Vec<isize> {
    let mut output: Vec<isize> = Vec::with_capacity(input.len());
    let mut sorted_input: Vec<isize> = Vec::with_capacity(input.len());

    sorted_input.extend(input);
    sorted_input.sort_unstable();

    for chunk in sorted_input.chunks_exact(2) {
        output.push(chunk[1]);
        output.push(chunk[0]);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec!(2, 5, 3, 4);
        let expected = vec!(3, 2, 5, 4);
        assert_eq!(number_game(&input), expected);
    }

    #[test]
    fn test2() {
        let input = vec!(9, 4, 1, 3, 6, 4, 6, 1);
        let expected = vec!(1, 1, 4, 3, 6, 4, 9, 6);
        assert_eq!(number_game(&input), expected);
    }

    #[test]
    fn test3() {
        let input = vec!(1, 2, 2, 3);
        let expected = vec!(2, 1, 3, 2);
        assert_eq!(number_game(&input), expected);
    }
}
