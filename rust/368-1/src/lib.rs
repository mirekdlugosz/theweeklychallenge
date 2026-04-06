pub fn make_it_bigger(str: &str, char: char) -> String {
    let chars: Vec<char> = str.chars().collect();
    chars
        .iter()
        .enumerate()
        .filter_map(|(idx, ch)| (*ch == char).then_some(idx))
        .map(|pos| {
            let mut attempt = chars.clone();
            attempt.remove(pos);
            attempt.iter().collect::<String>()
        })
        .max_by_key(|n| n.parse::<usize>().unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = make_it_bigger("15456", '5');
        assert_eq!(result, "1546");
    }

    #[test]
    fn test2() {
        let result = make_it_bigger("7332", '3');
        assert_eq!(result, "732");
    }

    #[test]
    fn test3() {
        let result = make_it_bigger("2231", '2');
        assert_eq!(result, "231");
    }

    #[test]
    fn test4() {
        let result = make_it_bigger("543251", '5');
        assert_eq!(result, "54321");
    }

    #[test]
    fn test5() {
        let result = make_it_bigger("1921", '1');
        assert_eq!(result, "921");
    }
}
