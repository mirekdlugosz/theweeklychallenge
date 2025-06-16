use time::Date;
use time::macros::format_description;

pub fn day_of_year(date: &str) -> usize {
    let fmt = format_description!("[year]-[month]-[day]");
    Date::parse(date, fmt).map_or(0, time::Date::ordinal).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = day_of_year("2025-02-02");
        assert_eq!(result, 33);
    }

    #[test]
    fn test2() {
        let result = day_of_year("2025-04-10");
        assert_eq!(result, 100);
    }

    #[test]
    fn test3() {
        let result = day_of_year("2025-09-07");
        assert_eq!(result, 250);
    }

    #[test]
    fn test4() {
        let result = day_of_year("2024-04-10");
        assert_eq!(result, 101);
    }
}
