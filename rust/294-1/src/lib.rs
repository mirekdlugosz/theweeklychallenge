use std::collections::HashSet;

pub fn consecutive_sequence(ints: &[isize]) -> isize {
    let mut ints_set: HashSet<&isize> = HashSet::new();
    let mut min_int = isize::MAX;
    let mut max_int = isize::MIN;

    for int in ints.iter() {
        ints_set.insert(int);
        if &min_int > int {
            min_int = *int;
        }
        if int > &max_int {
            max_int = *int;
        }
    }

    let mut i = min_int;
    let mut longest_seq = -1;
    let mut current_seq = 0;

    // since we set longest_seq in else clause, we need to ensure
    // we hit it after reaching the highest number, which may be at the end
    // of longest sequence
    // technically this won't work if highest number happens to be isize::MAX
    while (max_int + 1) >= i {
        if ints_set.contains(&i) {
            current_seq += 1;
        } else {
            // there's always going to be a sequence of 1 number, unless we were
            // given empty iterable (which is an edge case we don't handle)
            if current_seq > 1 && current_seq > longest_seq {
                longest_seq = current_seq;
            }
            current_seq = 0;
        }
        i += 1;
    }

    longest_seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = consecutive_sequence(&[10, 4, 20, 1, 3, 2]);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let result = consecutive_sequence(&[0, 6, 1, 8, 5, 2, 4, 3, 0, 7]);
        assert_eq!(result, 9);
    }

    #[test]
    fn test3() {
        let result = consecutive_sequence(&[10, 30, 20]);
        assert_eq!(result, -1);
    }
}
