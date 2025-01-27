fn sum_from_position(ints: &[usize], pos: usize) -> usize {
    let mut total: usize = 0;
    let mut length: usize = 1;
    loop {
        let end_pos = pos + length;
        let subslice = ints.get(pos..end_pos);

        match subslice {
            Some(sslice) => total += sslice.iter().sum::<usize>(),
            None => break,
        };
        length += 2;
    }
    total
}

pub fn odd_sum(ints: &[usize]) -> usize {
    ints
        .iter()
        .enumerate()
        .map(|(idx, _)| sum_from_position(ints, idx))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = odd_sum(&[2, 5, 3, 6, 4]);
        assert_eq!(result, 77);
    }

    #[test]
    fn test2() {
        let result = odd_sum(&[1, 3]);
        assert_eq!(result, 4);
    }
}
