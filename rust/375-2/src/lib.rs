pub fn k_beauty(num: usize, k: usize) -> usize {
    let string = format!("{num}");
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(k)
        .filter(|w| {
            let div: usize = w.iter().collect::<String>().parse().unwrap();
            num.rem_euclid(div) == 0
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = k_beauty(240, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = k_beauty(1020, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let result = k_beauty(444, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = k_beauty(17, 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn test5() {
        let result = k_beauty(123, 1);
        assert_eq!(result, 2);
    }
}
