static DAYS: [&str; 31] = [
    "1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th", "13th",
    "14th", "15th", "16th", "17th", "18th", "19th", "20th", "21st", "22nd", "23rd", "24th", "25th",
    "26th", "27th", "28th", "29th", "30th", "31st",
];

static MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub fn format_date(date: &str) -> String {
    let mut i = date.split_whitespace();
    let (mut day, mut month, mut year): (usize, usize, String) = (0, 0, String::new());
    if let Some(formatted_day) = i.next()
        && let Some(idx_day) = DAYS.iter().position(|&d| d == formatted_day)
    {
        day = idx_day + 1;
    }
    if let Some(formatted_month) = i.next()
        && let Some(idx_month) = MONTHS.iter().position(|&m| m == formatted_month)
    {
        month = idx_month + 1;
    }
    if let Some(formatted_year) = i.next() {
        year = formatted_year.to_string();
    }
    format!("{year}-{month:02}-{day:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = format_date("1st Jan 2025");
        assert_eq!(result, "2025-01-01");
    }

    #[test]
    fn test_2() {
        let result = format_date("22nd Feb 2025");
        assert_eq!(result, "2025-02-22");
    }

    #[test]
    fn test_3() {
        let result = format_date("15th Apr 2025");
        assert_eq!(result, "2025-04-15");
    }

    #[test]
    fn test_4() {
        let result = format_date("23rd Oct 2025");
        assert_eq!(result, "2025-10-23");
    }

    #[test]
    fn test_5() {
        let result = format_date("31st Dec 2025");
        assert_eq!(result, "2025-12-31");
    }
}
