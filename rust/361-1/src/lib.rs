static FIBONACCI: [u8; 12] = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

fn generate_indexes(
    i: usize,
    current: &mut Vec<usize>,
    n: usize,
    result: &mut Vec<Vec<usize>>,
) {
    if i > n {
        result.push(current.clone());
        return;
    }
    current.push(i);
    generate_indexes(i + 2, current, n, result);
    current.pop();

    generate_indexes(i + 1, current, n, result);
}

pub fn zeckendorf_representation(int: u8) -> Vec<u8> {
    let last_idx = FIBONACCI.iter().position(|&f| f > int).unwrap() - 1;
    let mut current: Vec<usize> = Vec::new();
    let mut result: Vec<Vec<usize>> = Vec::new();
    generate_indexes(0, &mut current, last_idx, &mut result);
    result.sort_by_cached_key(Vec::len);

    for candidate in result {
        let mut fibonacci: Vec<u8> = candidate
            .iter()
            .filter_map(|idx| FIBONACCI.get(*idx))
            .copied()
            .collect();
        let candidate_sum: u8 = fibonacci.iter().sum();
        if candidate_sum == int {
            fibonacci.sort_unstable();
            fibonacci.reverse();
            return fibonacci;
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = zeckendorf_representation(4);
        assert_eq!(result, [3, 1]);
    }

    #[test]
    fn test2() {
        let result = zeckendorf_representation(12);
        assert_eq!(result, [8, 3, 1]);
    }

    #[test]
    fn test3() {
        let result = zeckendorf_representation(20);
        assert_eq!(result, [13, 5, 2]);
    }

    #[test]
    fn test4() {
        let result = zeckendorf_representation(96);
        assert_eq!(result, [89, 5, 2]);
    }

    #[test]
    fn test5() {
        let result = zeckendorf_representation(100);
        assert_eq!(result, [89, 8, 3]);
    }
}
