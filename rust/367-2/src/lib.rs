// this is trivial - does the later event start time begin before earlier event end time?
// The only interesting things are happening around midnight - if even starts at 23:30 and 
// ends at 00:30, then in our parsing function we need to tell it that "00 is realy 24".
// But then we need to tell the same thing when parsing second event, because if it starts
// at midnight, we need to treat it as 24. And that applies to 01, 02 etc.
// Even more interesting is a case when event1 starts at 23:30, and event2 starts at 23:45,
// because now we **can't** offset 23 by another 24 hours. But where does that end? Is 22:00
// one and a half hour earlier than 23:30, or twenty two hours later?
// Without dates, this is very arbitrary.
// I considered implementing that briefly, but ultimately decided against it. Naive approach
// follows.

fn parse_time(time: &str) -> usize {
    let hours: usize = time.get(0..=1).unwrap().parse::<usize>().unwrap();
    let minutes: usize = time.get(3..=4).unwrap().parse::<usize>().unwrap();
    hours * 60 + minutes
}

pub fn conflict_events(event1: (&str, &str), event2: (&str, &str)) -> bool {
    // we assume event2 always starts later than event1. Which is probably very optimistic
    // assumption
    let end = parse_time(event1.1);
    let start = parse_time(event2.0);
    end > start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = conflict_events(("10:00", "12:00"), ("11:00", "13:00"));
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = conflict_events(("09:00", "10:30"), ("10:30", "12:00"));
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = conflict_events(("14:00", "15:30"), ("14:30", "16:00"));
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let result = conflict_events(("08:00", "09:00"), ("09:01", "10:00"));
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let result = conflict_events(("23:30", "00:30"), ("00:00", "01:00"));
        assert_eq!(result, true);
    }
}
