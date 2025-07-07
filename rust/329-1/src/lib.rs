use std::collections::HashSet;

pub fn counter_integers(str: &str) -> HashSet<usize> {
    str.split(|c: char| !c.is_ascii_digit())
        .filter_map(|num| num.parse::<usize>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = counter_integers("the1weekly2challenge2");
        assert_eq!(result, [1, 2].into());
    }

    #[test]
    fn test2() {
        let result = counter_integers("go21od1lu5c7k");
        assert_eq!(result, [21, 1, 5, 7].into());
    }

    #[test]
    fn test3() {
        let result = counter_integers("4p3e2r1l");
        assert_eq!(result, [4, 3, 2, 1].into());
    }
}
