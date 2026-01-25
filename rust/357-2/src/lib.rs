use std::collections::HashSet;

pub fn unique_fraction_generator(num: usize) -> Vec<(usize, usize)> {
    let mut storage: HashSet<(usize, usize)> = HashSet::with_capacity(num ^ 2);
    for numerator in 1..=num {
        for denominator in 1..=num {
            let fraction: f32 = (numerator as f32) / (denominator as f32);
            let mut has_close_fraction = false;
            for old_key in storage.iter() {
                let old_fraction: f32 = (old_key.0 as f32) / (old_key.1 as f32);
                if 1e-5 > (old_fraction - fraction).abs() {
                    has_close_fraction = true;
                    break;
                }
            }
            if has_close_fraction {
                continue;
            }
            storage.insert((numerator, denominator));
        }
    }
    let mut out = storage.iter().collect::<Vec<_>>();
    out.sort_unstable_by(|a, b| {
        let one = (a.0 as f32) / (a.1 as f32);
        let two = (b.0 as f32) / (b.1 as f32);
        one.total_cmp(&two)
    });
    out.into_iter().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = unique_fraction_generator(3);
        let expected = vec![(1, 3), (1, 2), (2, 3), (1, 1), (3, 2), (2, 1), (3, 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = unique_fraction_generator(4);
        let expected = vec![
            (1, 4),
            (1, 3),
            (1, 2),
            (2, 3),
            (3, 4),
            (1, 1),
            (4, 3),
            (3, 2),
            (2, 1),
            (3, 1),
            (4, 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = unique_fraction_generator(1);
        let expected = vec![(1, 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let result = unique_fraction_generator(6);
        let expected = vec![
            (1, 6),
            (1, 5),
            (1, 4),
            (1, 3),
            (2, 5),
            (1, 2),
            (3, 5),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6),
            (1, 1),
            (6, 5),
            (5, 4),
            (4, 3),
            (3, 2),
            (5, 3),
            (2, 1),
            (5, 2),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let result = unique_fraction_generator(5);
        let expected = vec![
            (1, 5),
            (1, 4),
            (1, 3),
            (2, 5),
            (1, 2),
            (3, 5),
            (2, 3),
            (3, 4),
            (4, 5),
            (1, 1),
            (5, 4),
            (4, 3),
            (3, 2),
            (5, 3),
            (2, 1),
            (5, 2),
            (3, 1),
            (4, 1),
            (5, 1),
        ];
        assert_eq!(result, expected);
    }
}
