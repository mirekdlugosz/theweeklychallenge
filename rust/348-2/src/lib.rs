fn to_minutes(input: &str) -> Option<usize> {
    let hours = (input.get(0..2)?).parse::<usize>().ok()?;
    let minutes = (input.get(3..5)?).parse::<usize>().ok()?;
    Some(hours.saturating_mul(60).saturating_add(minutes))
}

pub fn covert_time(source: &str, target: &str) -> usize {
    let source = to_minutes(source).unwrap_or(0);
    let mut target = to_minutes(target).unwrap_or(0);
    if source > target {
        target = target.saturating_add(24 * 60);
    }
    if source == target {
        return 0;
    }

    let mut changes = 0;
    let mut difference = target.saturating_sub(source);

    for step in [60, 15, 5, 1] {
        changes += difference.div_euclid(step);
        difference = difference.rem_euclid(step);
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = covert_time("02:30", "02:45");
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = covert_time("11:55", "12:15");
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let result = covert_time("09:00", "13:00");
        assert_eq!(result, 4);
    }

    #[test]
    fn test4() {
        let result = covert_time("23:45", "00:30");
        assert_eq!(result, 3);
    }

    #[test]
    fn test5() {
        let result = covert_time("14:20", "15:25");
        assert_eq!(result, 2);
    }
}
