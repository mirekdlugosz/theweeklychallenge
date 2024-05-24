#[derive(Debug)]
enum EqualizerError {
    EmptyArray,
}


fn get_differences(ints: &[isize]) -> Result<Vec<isize>, EqualizerError> {
    let max = ints.iter().max().ok_or(EqualizerError::EmptyArray)?;

    let differences = ints
        .iter()
        .map(|i| max - i);

    Ok(differences.collect())
}

fn equalize_cost_x_only(ints: &[isize], x: isize) -> isize {
    let steps_count: isize = ints.iter().sum();
    steps_count * x
}

fn equalize_cost_xy(ints: &[isize], x: isize, y: isize) -> isize {
    let mut ints_array = ints.to_vec();
    let mut total_cost: isize = 0;
    loop {
        let remaining = ints_array
            .iter()
            .filter(|n| **n != 0)
            .count();

        match remaining {
            0 => break,
            1 => {
                total_cost += equalize_cost_x_only(&ints_array, x);
                break
            },
            _ => {
                ints_array.sort_unstable();
                let min_idx = ints_array.iter().position(|n| *n != 0).unwrap();
                let max_idx_rev = ints_array.iter().rev().position(|n| *n != 0).unwrap();
                // len() is 1 higher than index of last element
                let max_idx = ints_array.len() - max_idx_rev - 1;

                ints_array[min_idx] -= 1;
                ints_array[max_idx] -= 1;
                total_cost += y;
            },
        }
    }

    total_cost
}

pub fn equalize_cost(ints: &[isize], x: isize, y: isize) -> isize {
    let differences = get_differences(ints).unwrap();

    if y > (2 * x) {
        equalize_cost_x_only(&differences, x)
    } else {
        equalize_cost_xy(&differences, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ints = vec!(4, 1);
        let x = 3;
        let y = 2;
        assert_eq!(equalize_cost(&ints, x, y), 9);
    }

    #[test]
    fn test2() {
        let ints = vec!(2, 3, 3, 3, 5);
        let x = 2;
        let y = 1;
        assert_eq!(equalize_cost(&ints, x, y), 6);
    }

    #[test]
    fn test3() {
        let ints = vec!(2, 4, 2);
        let x = 2;
        let y = 1;
        assert_eq!(equalize_cost(&ints, x, y), 2);
    }

    #[test]
    fn test4() {
        let ints = vec!(2, 3, 3, 3, 5);
        let x = 2;
        let y = 6;
        assert_eq!(equalize_cost(&ints, x, y), 18);
    }

    #[test]
    fn test5() {
        let ints = vec!(-2, -3, 1, 3);
        let x = 1;
        let y = 2;
        assert_eq!(equalize_cost(&ints, x, y), 13);
    }
}
