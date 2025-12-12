pub fn meeting_point(path: &str) -> bool {
    path.chars()
        .scan((0, 0), |position, change| {
            match change {
                'U' => position.1 += 1,
                'D' => position.1 -= 1,
                'R' => position.0 += 1,
                'L' => position.0 -= 1,
                _ => return None,
            }
            Some(*position)
        })
        .any(|position| position == (0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = meeting_point("ULD");
        assert_eq!(result, false);
    }

    #[test]
    fn test2() {
        let result = meeting_point("ULDR");
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = meeting_point("UUURRRDDD");
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = meeting_point("UURRRDDLLL");
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = meeting_point("RRUULLDDRRUU");
        assert_eq!(result, true);
    }
}
