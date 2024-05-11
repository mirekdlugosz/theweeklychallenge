pub fn magic_number(x: &[isize], y: &[isize]) -> isize {
    // It's reasonable to assume that this function *should* check
    // if there is a magic number to begin with, and allow for x and y
    // to be reversed. But these cases are not specified.
    let left = x.iter().min().unwrap();
    let right = y.iter().min().unwrap();

    right - left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let x = vec!(3, 7, 5);
        let y = vec!(9, 5, 7);
        assert_eq!(magic_number(&x, &y), 2);
    }

    #[test]
    fn test2() {
        let x = vec!(1, 2, 1);
        let y = vec!(5, 4, 4);
        assert_eq!(magic_number(&x, &y), 3);
    }

    #[test]
    fn test3() {
        let x = vec!(2);
        let y = vec!(5);
        assert_eq!(magic_number(&x, &y), 3);
    }
}
