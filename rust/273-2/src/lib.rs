pub fn b_after_a(str: &str) -> bool {
    let b_pos = str
        .chars()
        .position(|x| x == 'b');
    let b_pos = match b_pos {
        Some(p) => p,
        None => return false,
    };

    str
        .chars()
        .skip(b_pos)
        .position(|x| x == 'a')
        .is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(b_after_a("aabb"), true);
    }

    #[test]
    fn test2() {
        assert_eq!(b_after_a("abab"), false);
    }

    #[test]
    fn test3() {
        assert_eq!(b_after_a("aaa"), false);
    }

    #[test]
    fn test4() {
        assert_eq!(b_after_a("bbb"), true);
    }
}
