pub fn largest_number(ints: &[usize]) -> u64 {
    let mut sorted: Vec<String> = ints.iter().map(usize::to_string).collect();
    // this is mostly sorting numbers as a string.
    // the problem is in one special case - where numbers are of unequal length
    // and one begins with another. In that case we need "34" > "3" > "31",
    // which *seems* to fit a pretty convoluted rule of:
    // "take the longer number and remove the shorter one from the beginning;
    // if the remainder is string-smaller than the shorter, then consider
    // shorter to be greater"
    // it makes me think I am missing something more obvious here
    sorted.sort_unstable_by(|a, b| {
        if a.len() != b.len() {
            if a.starts_with(b) {
                return b.cmp(&a.trim_start_matches(b).to_string());
            }
            if b.starts_with(a) {
                return a.cmp(&b.trim_start_matches(a).to_string());
            }
        }
        b.cmp(a)
    });
    sorted.join("").parse::<u64>().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = largest_number(&[20, 3]);
        assert_eq!(result, 320);
    }

    #[test]
    fn test2() {
        let result = largest_number(&[3, 30, 34, 5, 9]);
        assert_eq!(result, 9534330);
    }
}
