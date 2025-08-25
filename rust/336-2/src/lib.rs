pub fn final_score(scores: &[&str]) -> isize {
    let mut stack: Vec<isize> = Vec::with_capacity(scores.len());

    for score in scores {
        match *score {
            "C" => {
                let _ = stack.pop();
            }
            "D" => {
                if let Some(elem) = stack.last() {
                    stack.push(*elem * 2);
                }
            }
            "+" => {
                if let Some(chunk) = stack.last_chunk::<2>() {
                    stack.push(chunk.iter().sum());
                }
            }
            _ => {
                if let Ok(num) = score.parse::<isize>() {
                    stack.push(num);
                }
            }
        }
    }
    stack.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = final_score(&["5", "2", "C", "D", "+"]);
        assert_eq!(result, 30);
    }

    #[test]
    fn test2() {
        let result = final_score(&["5", "-2", "4", "C", "D", "9", "+", "+"]);
        assert_eq!(result, 27);
    }

    #[test]
    fn test3() {
        let result = final_score(&["7", "D", "D", "C", "+", "3"]);
        assert_eq!(result, 45);
    }

    #[test]
    fn test4() {
        let result = final_score(&["-5", "-10", "+", "D", "C", "+"]);
        assert_eq!(result, -55);
    }

    #[test]
    fn test5() {
        let result = final_score(&["3", "6", "+", "D", "C", "8", "+", "D", "-2", "C", "+"]);
        assert_eq!(result, 128);
    }
}
