pub fn tax_amount(income: f32, tax: &[(f32, f32)]) -> f32 {
    let mut tax_to_pay = 0.0;
    let mut previous_upper_bound = 0.0;
    for (upper_bound, tax_rate) in tax {
        let final_bracket = *upper_bound >= income;
        let amount_to_tax =
            if final_bracket { income } else { *upper_bound } - previous_upper_bound;
        tax_to_pay += amount_to_tax * (tax_rate / 100.0);
        if final_bracket {
            break;
        }
        previous_upper_bound = *upper_bound;
    }
    tax_to_pay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = tax_amount(10.0, &[(3.0, 50.0), (7.0, 10.0), (12.0, 25.0)]);
        assert_eq!(result, 2.65);
    }

    #[test]
    fn test2() {
        let result = tax_amount(2.0, &[(1.0, 0.0), (4.0, 25.0), (5.0, 50.0)]);
        assert_eq!(result, 0.25);
    }

    #[test]
    fn test3() {
        let result = tax_amount(0.0, &[(2.0, 50.0)]);
        assert_eq!(result, 0.0);
    }
}
