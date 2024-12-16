use itertools::Itertools;

fn is_beautiful_arrangements(arrangement: &[usize]) -> bool {
    arrangement.iter()
        .enumerate()
        .all(|(i, val)| {
            let i = i + 1;
            val.rem_euclid(i) == 0 || i.rem_euclid(*val) == 0
        })
}

pub fn count_beautiful_arrangements(n: usize) -> usize {
    let array: Vec<usize> = (1..=n).collect();
    array.into_iter()
        .permutations(n)
        .filter(|arr| is_beautiful_arrangements(arr))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = count_beautiful_arrangements(2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = count_beautiful_arrangements(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let result = count_beautiful_arrangements(10);
        assert_eq!(result, 700);
    }
}
