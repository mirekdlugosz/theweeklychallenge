use std::collections::HashMap;


pub fn max_frequency(ints: &[usize]) -> usize {
    let mut frequencies: HashMap<usize, usize> = HashMap::with_capacity(ints.len());

    for value in ints {
        frequencies.entry(*value).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let max_freq = frequencies.values().max().unwrap_or(&0);

    frequencies.values()
        .filter(|v| *v == max_freq)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = max_frequency(&[1, 2, 2, 4, 1, 5]);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let result = max_frequency(&[1, 2, 3, 4, 5]);
        assert_eq!(result, 5);
    }
}
