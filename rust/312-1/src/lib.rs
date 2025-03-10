use std::collections::HashMap;

fn get_wheel() -> HashMap<char, usize> {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    letters
        .chars()
        .enumerate()
        .map(|(idx, ch)| (ch, idx + 1))
        .collect()
}

pub fn minimum_time(string: &str) -> usize {
    let mut total_time = string.chars().count();
    let wheel = get_wheel();
    let wheel_size = wheel.len();
    let mut pointer = wheel.get(&'a').unwrap_or(&1);

    for letter in string.chars() {
        let target_pointer = wheel.get(&letter).unwrap();
        if target_pointer == pointer {
            continue;
        }
        let non_wrapping = target_pointer.max(pointer) - target_pointer.min(pointer);
        let wrapping = (wheel_size - target_pointer.max(pointer)) + target_pointer.min(pointer);

        total_time += non_wrapping.min(wrapping);
        pointer = target_pointer;
    }

    total_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = minimum_time("abc");
        assert_eq!(result, 5);
    }

    #[test]
    fn test2() {
        let result = minimum_time("bza");
        assert_eq!(result, 7);
    }

    #[test]
    fn test3() {
        let result = minimum_time("zjpc");
        assert_eq!(result, 34);
    }
}
