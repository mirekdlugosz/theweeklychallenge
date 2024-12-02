fn calc_right_interval(starts: &[usize], end: &usize) -> isize {
    starts.iter()
        .enumerate()
        .filter_map(|(idx, start)| {
            match start >= end {
                true => Some((idx, start)),
                false => None
            }
        })
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map_or(-1, |(idx, _)| idx.try_into().unwrap_or(-1))
}

pub fn right_interval(intervals: &[(usize, usize)]) -> Vec<isize> {
    let starts: Vec<_> = intervals.iter()
        .map(|interval| interval.0)
        .collect();

    intervals.iter()
        .map(|interval| calc_right_interval(&starts, &interval.1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let intervals = [
            (3,4), (2,3), (1,2),
        ];
        let expected = vec!(-1, 0, 1);
        assert_eq!(right_interval(&intervals), expected);
    }

    #[test]
    fn test2() {
        let intervals = [
            (1,4), (2,3), (3,4),
        ];
        let expected = vec!(-1, 2, -1);
        assert_eq!(right_interval(&intervals), expected);
    }

    #[test]
    fn test3() {
        let intervals = [(1,2)];
        let expected = vec!(-1);
        assert_eq!(right_interval(&intervals), expected);
    }

    #[test]
    fn test4() {
        let intervals = [
            (1,4), (2,2), (3,4),
        ];
        let expected = vec!(-1, 1, -1);
        assert_eq!(right_interval(&intervals), expected);
    }
}
