pub fn equal_list(arr1: &[&str], arr2: &[&str]) -> bool {
    arr1.join("") == arr2.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = equal_list(&["a", "bc"], &["ab", "c"]);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = equal_list(&["a", "b", "c"], &["a", "bc"]);
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = equal_list(&["a", "bc"], &["a", "c", "b"]);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = equal_list(&["ab", "c", ""], &["", "a", "bc"]);
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = equal_list(&["p", "e", "r", "l"], &["perl"]);
        assert_eq!(result, true);
    }
}
