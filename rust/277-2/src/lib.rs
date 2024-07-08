#[derive(Debug, Eq)]
pub struct Pair(isize, isize);

// while we could use derived PartialEq for provided tests to pass,
// implementing the fact that Pair is essentially unordered is easy
// enough
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0
         && self.1 == other.1)
        || (self.0 == other.1
         && self.1 == other.0)
    }
}

fn is_pair_strong(pair: &Pair) -> bool {
    let diff = pair.0.abs_diff(pair.1);
    if diff == 0 {
        return false;
    }

    let min = pair.0.min(pair.1);

    diff < min.try_into().unwrap()
}

pub fn strong_pairs(ints: &[isize]) -> Vec<Pair> {
    let mut output = Vec::new();

    for (i_idx, i_value) in ints.iter().enumerate() {
        for j_value in ints.iter().skip(i_idx + 1) {
            let pair = Pair(*i_value, *j_value);
            if ! is_pair_strong(&pair) {
                continue
            }
            if output.contains(&pair) {
                continue
            }
            output.push(pair);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ints = [1, 2, 3, 4, 5];
        let expected = vec!(
            Pair(2, 3),
            Pair(3, 4),
            Pair(3, 5),
            Pair(4, 5),
        );
        assert_eq!(strong_pairs(&ints), expected);
    }

    #[test]
    fn test2() {
        let ints = [5, 7, 1, 7];
        let expected = vec!(
            Pair(5, 7),
        );
        assert_eq!(strong_pairs(&ints), expected);
    }
}
