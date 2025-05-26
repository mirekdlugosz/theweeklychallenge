pub fn increment_decrement(operations: &[&str]) -> isize {
    operations
        .iter()
        .map(|e| match *e {
            "--x" | "x--" => -1,
            "++x" | "x++" => 1,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = increment_decrement(&["--x", "x++", "x++"]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = increment_decrement(&["x++", "++x", "x++"]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let result = increment_decrement(&["x++", "++x", "--x", "x--"]);
        assert_eq!(result, 0);
    }
}
