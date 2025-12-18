pub fn special_average(ints: &[isize]) -> isize {
    let mut arr_min = isize::MAX;
    let mut arr_max = isize::MIN;
    for int in ints {
        if *int > arr_max {
            arr_max = *int;
        }
        if arr_min > *int {
            arr_min = *int;
        }
    }

    let filtered: Vec<isize> = ints
        .iter()
        .filter(|&int| !(*int == arr_min || *int == arr_max))
        .copied()
        .collect();
    let total: isize = filtered.iter().sum();
    let num = filtered.len() as isize;
    if num == 0 { 0 } else { total.div_euclid(num) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = special_average(&[8000, 5000, 6000, 2000, 3000, 7000]);
        assert_eq!(result, 5250);
    }

    #[test]
    fn test2() {
        let result = special_average(&[100_000, 80_000, 110_000, 90_000]);
        assert_eq!(result, 95_000);
    }

    #[test]
    fn test3() {
        let result = special_average(&[2500, 2500, 2500, 2500]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = special_average(&[2500]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test5() {
        let result = special_average(&[1000, 2000, 3000, 4000, 5000, 6000]);
        assert_eq!(result, 3500);
    }
}
