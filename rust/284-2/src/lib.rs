pub fn relative_sort(list1: &[isize], list2: &[isize]) -> Vec<isize> {
    // This is naive version. The problem calls for removing elements
    // from list1 as we find them in list2 order. We only need to figure
    // out how to modify list1 as we iterate over it. This will mean
    // that subsequent list2 iterations are slightly faster, as list1
    // becomes smaller. What would be left in list1 after first loop
    // are things that do not appear in list2, so we could just sort it
    // and append to output.
    // There's probably also some clever approach that only moves things
    // inside copy of list1 as we go through list2, and then deals with
    // the rest.
    // But naive version works, is easy to understand and took less time
    // to write than this comment, so it goes.
    let mut output: Vec<isize> = Vec::new();
    let mut the_rest: Vec<isize> = Vec::new();

    for ordered_elem in list2 {
        for list1_elem in list1 {
            if list1_elem == ordered_elem {
                output.push(*ordered_elem);
            }
        }
    }

    for list1_elem in list1 {
        if ! list2.contains(list1_elem) {
            the_rest.push(*list1_elem);
        }
    }

    the_rest.sort_unstable();

    output.append(&mut the_rest);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = relative_sort(
            &[2, 3, 9, 3, 1, 4, 6, 7, 2, 8, 5],
            &[2, 1, 4, 3, 5, 6]
        );
        let expected: Vec<isize> = [2, 2, 1, 4, 3, 3, 5, 6, 7, 8, 9].into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = relative_sort(
            &[3, 3, 4, 6, 2, 4, 2, 1, 3],
            &[1, 3, 2]
        );
        let expected: Vec<isize> = [1, 3, 3, 3, 2, 2, 4, 4, 6].into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let result = relative_sort(
            &[3, 0, 5, 0, 2, 1, 4, 1, 1],
            &[1, 0, 3, 2]
        );
        let expected: Vec<isize> = [1, 1, 1, 0, 0, 3, 2, 4, 5].into();
        assert_eq!(result, expected);
    }
}
