pub fn is_luhns(input: &str) -> bool {
    let nums: Vec<u32> = input.chars()
        .rev()
        .filter_map(|ch| ch.to_digit(10))
        .collect();

    let payload = nums.first().unwrap();

    let sum: u32 = nums.iter()
        .enumerate()
        .skip(1)
        .map(|(idx, i)| {
            match idx.rem_euclid(2) == 1 {
                false => *i,
                true => {
                    let doubled = *i * 2;
                    match doubled > 9 {
                        false => doubled,
                        true => doubled.to_string()
                            .chars()
                            .filter_map(|ch| ch.to_digit(10))
                            .sum(),
                    }
                }
            }
        })
        .sum();

    let real_payload = 10 - sum.rem_euclid(10);
    *payload == real_payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = is_luhns("17893729974");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = is_luhns("4137 8947 1175 5904");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = is_luhns("4137 8974 1175 5904");
        assert_eq!(result, false);
    }
}
