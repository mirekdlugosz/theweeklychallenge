pub fn string_reduction(word: &str) -> String {
    let mut out = word.to_string();
    loop {
        let duplicate_indices: Vec<usize> = out
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .enumerate()
            .filter_map(|(idx, window)| {
                let first = window.first()?;
                let second = window.last()?;
                if first == second { Some(idx) } else { None }
            })
            .collect();

        if duplicate_indices.is_empty() {
            break;
        }

        // The problem is that if we have a sequence of three letters, then duplicate_indices
        // will find two letter-length duplicates at index 0 and 1, and we will remove all
        // three of them below. But three letter sequence should be a single letter after
        // removal.
        // We basically want a filter that can look into previous item and return false
        // if current item is larger by one than a previous item.
        // fold (reduce) is a good stand-in for this filter, but it will do a lot of
        // allocations...
        let duplicate_indices = duplicate_indices.iter().fold(vec![], |acc, &i| {
            if acc.is_empty() {
                return vec![i, i + 1];
            }

            let last = acc.last().unwrap();
            if *last == i {
                return acc;
            }
            let mut new_acc: Vec<usize> = acc.to_vec();
            new_acc.push(i);
            new_acc.push(i + 1);
            new_acc
        });

        let mut new_out: Vec<char> = out.chars().collect();
        for idx in duplicate_indices.iter().rev() {
            new_out.remove(*idx);
        }
        out = new_out.iter().copied().collect();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = string_reduction("aabbccdd");
        assert_eq!(result, "");
    }

    #[test]
    fn test2() {
        let result = string_reduction("abccba");
        assert_eq!(result, "");
    }

    #[test]
    fn test3() {
        let result = string_reduction("abcdef");
        assert_eq!(result, "abcdef");
    }

    #[test]
    fn test4() {
        let result = string_reduction("aabbaeaccdd");
        assert_eq!(result, "aea");
    }

    #[test]
    fn test5() {
        let result = string_reduction("mississippi");
        assert_eq!(result, "m");
    }
}
