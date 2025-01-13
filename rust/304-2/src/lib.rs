pub fn maximum_average(ints: &[isize], n: usize) -> f64 {
    ints
        .windows(n)
        .map(|window| window.iter().sum::<isize>() as f64 / window.len() as f64)
        .max_by(f64::total_cmp)
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = maximum_average(&[1, 12, -5, -6, 50, 3], 4);
        assert_eq!(result, 12.75);
    }

    #[test]
    fn test2() {
        let result = maximum_average(&[5], 1);
        assert_eq!(result, 5.0);
    }
}
