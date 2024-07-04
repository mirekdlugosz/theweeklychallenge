#[derive(Debug, PartialEq)]
pub struct CompleteDayPair(usize, usize);

pub fn complete_days(hours: &[usize]) ->  Vec<CompleteDayPair> {
    let mut output = Vec::new();

    for (i_idx, i_value) in hours.iter().enumerate() {
        for j_value in hours.iter().skip(i_idx + 1) {
            let complete_pair = (i_value + j_value) % 24 == 0;
            if ! complete_pair {
                continue;
            }
            output.push(CompleteDayPair(*i_value, *j_value));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let hours = vec!(12, 12, 30, 24, 24);
        let expected = vec!(
            CompleteDayPair(12, 12),
            CompleteDayPair(24, 24),
        );
        assert_eq!(complete_days(&hours), expected);
    }

    #[test]
    fn test2() {
        let hours = vec!(72, 48, 24, 5);
        let expected = vec!(
            CompleteDayPair(72, 48),
            CompleteDayPair(72, 24),
            CompleteDayPair(48, 24),
        );
        assert_eq!(complete_days(&hours), expected);
    }

    #[test]
    fn test3() {
        let hours = vec!(12, 18, 24);
        let expected = vec!();
        assert_eq!(complete_days(&hours), expected);
    }
}
