pub fn sort_column(list: &[&str]) -> usize {
    let mut deleted_count = 0;
    let mut iterators = Vec::new();
    for string in list {
        iterators.push(string.chars());
    }

    'main: loop {
        let mut column = Vec::new();
        for col in &mut iterators {
            if let Some(ch) = col.next() {
                column.push(ch);
            } else {
                break 'main;
            }
        }

        if ! column.is_sorted() {
            deleted_count += 1;
        }
    }
    deleted_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = sort_column(&["swpc", "tyad", "azbe"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = sort_column(&["cba", "daf", "ghi"]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let result = sort_column(&["a", "b", "c"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = sort_column(&["abc", "bcd", "cde", "def"]);
        assert_eq!(result, 0);
    }
}
