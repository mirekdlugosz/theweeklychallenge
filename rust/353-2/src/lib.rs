const VALID_NAMES: [&str; 4] = ["electronics", "grocery", "pharmacy", "restaurant"];

pub fn validate_coupon(codes: &[&str], names: &[&str], status: &[&str]) -> Vec<bool> {
    codes
        .iter()
        .enumerate()
        .map(|(i, code)| {
            let all_chars_valid = code
                .chars()
                .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_');
            let i_name = names.get(i).unwrap_or(&"");
            let i_status = status.get(i).unwrap_or(&"false");

            all_chars_valid && VALID_NAMES.contains(i_name) && *i_status == "true"
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = validate_coupon(
            &["A123", "B_456", "C789", "D@1", "E123"],
            &[
                "electronics",
                "restaurant",
                "electronics",
                "pharmacy",
                "grocery",
            ],
            &["true", "false", "true", "true", "true"],
        );
        let expected = vec![true, false, true, false, true];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = validate_coupon(
            &["Z_9", "AB_12", "G01", "X99", "test"],
            &[
                "pharmacy",
                "electronics",
                "grocery",
                "electronics",
                "unknown",
            ],
            &["true", "true", "false", "true", "true"],
        );
        let expected = vec![true, true, false, true, false];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = validate_coupon(
            &["_123", "123", "", "Coupon_A", "Alpha"],
            &[
                "restaurant",
                "electronics",
                "electronics",
                "pharmacy",
                "grocery",
            ],
            &["true", "true", "false", "true", "true"],
        );
        let expected = vec![true, true, false, true, true];
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let result = validate_coupon(
            &["ITEM_1", "ITEM_2", "ITEM_3", "ITEM_4"],
            &["electronics", "electronics", "grocery", "grocery"],
            &["true", "true", "true", "true"],
        );
        let expected = vec![true, true, true, true];
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let result = validate_coupon(
            &["CAFE_X", "ELEC_100", "FOOD_1", "DRUG_A", "ELEC_99"],
            &[
                "restaurant",
                "electronics",
                "grocery",
                "pharmacy",
                "electronics",
            ],
            &["true", "true", "true", "true", "false"],
        );
        let expected = vec![true, true, true, true, false];
        assert_eq!(result, expected);
    }
}
