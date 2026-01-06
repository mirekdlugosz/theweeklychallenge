pub fn mountain_array(ints: &[isize]) -> bool {
    let mut max = isize::MIN;
    let mut max_positions = Vec::with_capacity(ints.len());
    for (idx, i) in ints.iter().enumerate() {
        if *i > max {
            max = *i;
            max_positions.clear();
            max_positions.push(idx);
            continue;
        }
        if *i == max {
            max_positions.push(idx);
        }
    }

    if max_positions.len() > 1 {
        return false;
    }

    let max_place = max_positions.first().unwrap_or(&0);

    if *max_place == 0 || (*max_place + 1) == ints.len() {
        return false;
    }

    if let Some(up_the_hill) = ints.get(0..*max_place) {
        let up_correct = up_the_hill.windows(2).all(|w| {
            let f = w.first().unwrap_or(&0);
            let l = w.last().unwrap_or(&0);
            l > f
        });
        if !up_correct {
            return false;
        }
    }

    if let Some(down_the_hill) = ints.get(*max_place..) {
        let down_correct = down_the_hill.windows(2).all(|w| {
            let f = w.first().unwrap_or(&0);
            let l = w.last().unwrap_or(&0);
            f > l
        });
        if !down_correct {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = mountain_array(&[1, 2, 3, 4, 5]);
        assert_eq!(result, false);
    }

    #[test]
    fn test2() {
        let result = mountain_array(&[0, 2, 4, 6, 4, 2, 0]);
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = mountain_array(&[5, 4, 3, 2, 1]);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let result = mountain_array(&[1, 3, 5, 5, 4, 2]);
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let result = mountain_array(&[1, 3, 2]);
        assert_eq!(result, true);
    }
}
