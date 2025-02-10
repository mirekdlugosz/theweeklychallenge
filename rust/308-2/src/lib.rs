pub fn decode_xor(encoded: &[isize], initial: isize) -> Vec<isize> {
    let mut out: Vec<isize> = [initial].to_vec();
    let mut last = initial;

    for encoded_num in encoded {
        last ^= *encoded_num;
        out.push(last);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = decode_xor(&[1, 2, 3], 1);
        assert_eq!(result, [1, 0, 2, 1].to_vec());
    }

    #[test]
    fn test2() {
        let result = decode_xor(&[6, 2, 7, 3], 4);
        assert_eq!(result, [4, 2, 0, 7, 4].to_vec());
    }
}
