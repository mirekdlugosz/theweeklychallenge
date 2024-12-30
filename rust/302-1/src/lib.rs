struct Elem {
    x: usize,
    y: usize,
}

impl From<&str> for Elem {
    fn from(value: &str) -> Self {
        let (all_x, all_y): (Vec<_>, Vec<_>) = value.chars()
            .partition(|v| *v == '0');
        Self {
            x: all_x.len(),
            y: all_y.len()
        }
    }
}

pub fn ones_and_zeroes(strs: &[&str], x: usize, y: usize) -> usize {
    // Basically: start with the entire vector and remove elements one by one until
    // what is left meets the criteria, or nothing is left.
    // But how to know which element should be removed?
    // Heuristic approach would be to remove the longest element. It would work
    // in provided test cases.
    // But what if we have too many 1s and the longest element is all 0s? Removing
    // it won't bring us closer to our goal.
    // The following algorithm is hopefully correct in larger number of cases:
    // 1. Calculate a total number of 0s (total_x) and 1s (total_y) in a vector
    // 2. For both x and y, calculate how far we are from meeting criteria
    // 3. Take condition (x or y) that is further from meeting criteria
    // 4. Find the element that contributes the most to that condition
    // 5. Remove element from the vector
    // 6. If we meet the criteria now, stop. If we don't, go back to step 1
    //
    // The implementation below is optimized to iterate over remaining elements
    // only once for each decision - we identify elements contributing the most
    // to x and y while calculating totals.
    let mut largest_set: Vec<Elem> = strs.iter()
        .map(|&s| Elem::from(s))
        .collect();

    let x: isize = isize::try_from(x).unwrap();
    let y: isize = isize::try_from(y).unwrap();

    while !largest_set.is_empty() {
        let mut total_x: usize = 0;
        let mut total_y: usize = 0;
        let mut most_x: usize = usize::MIN;
        let mut most_x_idx: Option<usize> = None;
        let mut most_y: usize = usize::MIN;
        let mut most_y_idx: Option<usize> = None;

        for (idx, elem) in largest_set.iter().enumerate() {
            total_x += elem.x;
            total_y += elem.y;

            if elem.x > most_x {
                most_x = elem.x;
                most_x_idx = Some(idx);
            }

            if elem.y > most_y {
                most_y = elem.y;
                most_y_idx = Some(idx);
            }
        }

        // negative numbers are more natural in this case and allow to make
        // later code more concise
        let above_limit_x = isize::try_from(total_x).unwrap_or(isize::MAX) - x;
        let above_limit_y = isize::try_from(total_y).unwrap_or(isize::MAX) - y;

        if 0 >= above_limit_x && 0 >= above_limit_y {
            break;
        }

        let elem_to_remove = if above_limit_x > above_limit_y {
            most_x_idx
        } else {
            most_y_idx
        };

        largest_set.swap_remove(elem_to_remove.unwrap());
    }

    largest_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = ones_and_zeroes(&["10", "0001", "111001", "1", "0"], 5, 3);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let result = ones_and_zeroes(&["10", "1", "0"], 1, 1);
        assert_eq!(result, 2);
    }
}
