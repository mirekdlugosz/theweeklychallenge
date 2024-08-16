pub fn good_integer(integer: usize) -> isize {
    let mut next_item = String::new();
    let intasstring = integer.to_string();
    let mut chars = intasstring.chars();

    let mut current_char = chars.next().unwrap();
    next_item.push(current_char);


    for ch in chars {
        if ch == current_char {
            next_item.push(ch);
            continue;
        }
        
        if next_item.len() == 3 {
            return next_item.as_str().parse::<isize>().unwrap_or(-1);
        }

        next_item.clear();
        next_item.push(ch);
        current_char = ch;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = good_integer(12344456);
        assert_eq!(result, 444);
    }

    #[test]
    fn test2() {
        let result = good_integer(1233334);
        assert_eq!(result, -1);
    }

    #[test]
    fn test3() {
        let result = good_integer(10020003);
        assert_eq!(result, 000);
    }
}
