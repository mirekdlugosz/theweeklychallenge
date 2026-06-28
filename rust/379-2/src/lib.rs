use convert_base::Convert;

// I find these entire "Armstrong numbers" very confusing.
// convert_base does not help by working with arrays with least-significant
// numbers at the beginning. I'm sure it makes sense in some way, but is
// super annoying for the problem I have here.
// About halfway through I gave up and just asked Copilot to give me
// code that fills the missing pieces.
// I am not ashamed. I find this exercise completely uninteresting.


// this is AI-generated, because this crate does stupid things
pub fn digits_lsf(mut n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    if n == 0 {
        out.push(0);
        return out;
    }
    while n > 0 {
        out.push((n % 10) as u32);
        n /= 10;
    }
    out
}
// end AI-generated

fn is_armstrong_number(n: u32, base: u32) -> Option<u32> {
    let mut convert = Convert::new(10, base as u64);
    let input = digits_lsf(n);
    let converted_arr = convert.convert::<u32, u32>(&input);
    // this is AI-generated, because this crate does stupid things
    let mut as_string = String::with_capacity(input.len());
    for &d in converted_arr.iter().rev() {
        let ch = match d {
            0..=9 => (b'0' + d as u8) as char,
            10..=35 => (b'A' + (d as u8 - 10)) as char,
            _ => '\0',
        };
        as_string.push(ch);
    }
    // end AI-generated
    let digits: u32 = as_string.chars().count().try_into().unwrap();
    let sum = as_string.chars().map(|d| {
        let num = u32::from_str_radix(&d.to_string(), base).unwrap_or(0);
        num.pow(digits)
    }).sum();
    if n == sum {
        Some(n)
    } else {
        None
    }
}

pub fn armstrong_number(base: u32, limit: u32) -> Vec<u32> {
    (0..=limit).filter_map(|n| is_armstrong_number(n, base)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = armstrong_number(10, 1000);
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 153, 370, 371, 407];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = armstrong_number(7, 1000);
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 10, 25, 32, 45, 133, 134, 152, 250];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = armstrong_number(16, 1000);
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 342, 371, 520, 584, 645];
        assert_eq!(result, expected);
    }
}
