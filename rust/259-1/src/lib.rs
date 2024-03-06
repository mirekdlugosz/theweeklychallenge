use chrono::{Datelike, Days, NaiveDate, Weekday};

pub fn day_offset(start: &str, offset: usize, bank_holidays: Option<&[&str]>) -> String {
    let bank_holidays_offset = match bank_holidays {
        None => 0,
        Some(bh) => bh.len(),
    };
    let weekends = (offset + bank_holidays_offset).div_ceil(7);
    let max_days = offset + bank_holidays_offset + (weekends * 2);
    let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap();
    (0..=max_days)
        .filter_map(|day_offset| {
            let new_date = start_date + Days::new(day_offset as u64);
            match new_date.weekday() {
                Weekday::Sat | Weekday::Sun => return None,
                _ => ()
            };
            let formatted = new_date.format("%Y-%m-%d").to_string();
            match bank_holidays {
                None => Some(formatted),
                Some(bh) => match bh.contains(&formatted.as_str()) {
                    true => None,
                    false => Some(formatted),
                }
            }
        })
        .nth(offset)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let bank_holidays = vec!("2018-07-03");
        let result = day_offset("2018-06-28", 3, Some(&bank_holidays));
        assert_eq!(result, "2018-07-04");
    }

    #[test]
    fn test2() {
        let result = day_offset("2018-06-28", 3, None);
        assert_eq!(result, "2018-07-03");
    }

    #[test]
    fn test3() {
        let result = day_offset("2024-02-26", 14, None);
        assert_eq!(result, "2024-03-15");
    }

    #[test]
    fn test4() {
        let result = day_offset("2024-02-26", 1, None);
        assert_eq!(result, "2024-02-27");
    }
}
