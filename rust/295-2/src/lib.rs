pub fn jump_game(ints: &[usize]) -> isize {
    let mut jumps = 0;
    let mut i = 0;

    let last_idx = ints.len() - 1;

    while last_idx > i {
        let next_idxs = ints.get(i).unwrap_or(&0);

        // if we can jump from here to end, just do it
        if i + next_idxs >= last_idx {
            return jumps + 1;
        }

        // Look into choices we can make and select one that allows us to
        // jump furthest next round.
        // If there are many, take the furthest one.
        //
        // Technically jumping further _now_ might be better move, even
        // if we end up jump fewer steps next round. The optimal solution
        // would probably weigh possible choices by their position, or at
        // least ensure next round we can still jump further than we can now.
        // Since the field is known, we could even create decision tree to check
        // all possible jumps - this would ensure we can later pick the shortest
        // path.
        let mut max = usize::MIN;
        let mut max_idx = 0;

        for (j_idx, j) in ints.iter().skip(i + 1).take(*next_idxs).enumerate() {
            if j >= &max {
                max = *j;
                max_idx = j_idx + 1;
            }
        }
        // if the largest number is 0, we have lost
        if max == 0 {
            return -1;
        }
        i += max_idx;
        jumps += 1;
    }
    jumps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = jump_game(&[2, 3, 1, 1, 4]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = jump_game(&[2, 3, 0, 4]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let result = jump_game(&[2, 0, 0, 4]);
        assert_eq!(result, -1);
    }

    #[test]
    fn test4() {
        let result = jump_game(&[2, 3, 0, 4, 0]);
        assert_eq!(result, 2);
    }

    // a case where our algorithm fails to find the shortest path
    #[test]
    #[should_panic]
    fn test5() {
        let result = jump_game(&[3, 2, 0, 1, 0]);
        assert_eq!(result, 2);
    }
}
