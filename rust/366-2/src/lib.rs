pub fn valid_times(time: &str) -> usize {
    let mut timei = time.chars();
    let h1 = timei.next().unwrap();
    let h2 = timei.next().unwrap();
    let _ = timei.next(); // colon
    let m1 = timei.next().unwrap();
    let m2 = timei.next().unwrap();

    let h_opts = match (h1, h2) {
        ('?', '?') => 24,
        ('?', '0' | '1' | '2' | '3') => 3,
        ('?', _) => 2,
        ('2', '?') => 4,
        (_, '?') => 10,
        (_, _) => 1,
    };
    let m_opts = match (m1, m2) {
        ('?', '?') => 60,
        ('?', _) => 6,
        (_, '?') => 10,
        (_, _) => 1,
    };
    h_opts * m_opts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = valid_times("?2:34");
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = valid_times("?4:?0");
        assert_eq!(result, 12);
    }

    #[test]
    fn test3() {
        let result = valid_times("??:??");
        assert_eq!(result, 1440);
    }

    #[test]
    fn test4() {
        let result = valid_times("?3:45");
        assert_eq!(result, 3);
    }

    #[test]
    fn test5() {
        let result = valid_times("2?:15");
        assert_eq!(result, 4);
    }

    #[test]
    fn test6() {
        let result = valid_times("?6:45");
        assert_eq!(result, 2);
    }
}
