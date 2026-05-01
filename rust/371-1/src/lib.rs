static LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

// This is naive algorithm that assumes '?' is never at first or last position
// It's a bit complex as we operate on three arrays: LETTERS (which we use as a map between element
// and index in alphabet), seq (which is array of letters), and change from index of one element in
// seq to next element
// It could be a bit more readable if we had two hashmaps - from char to index ('a' => 1 etc.) and
// from index to char (1 => 'a' etc.). But HashMaps can't be static, so we would have to create
// them at runtime.
pub fn missing_letter(seq: &[char]) -> char {
    // calculate the changes between known characters. This will be tuple of 2 numbers, either
    // same number twice or two different numbers
    let changes: Vec<_> = seq
        .windows(2)
        .filter_map(|window| {
            let a = window.first()?;
            let b = window.last()?;
            if let Some(a_pos) = LETTERS.iter().position(|&n| n == *a)
                && let Some(b_pos) = LETTERS.iter().position(|&n| n == *b)
            {
                return Some(b_pos.saturating_sub(a_pos));
            }
            None
        })
        .collect();
    // Find question mark in `seq`. Find index of previous element, and what that element is.
    let needle_seq_idx = seq.iter().position(|&n| n == '?').unwrap();
    let prev_seq_idx = needle_seq_idx.saturating_sub(1);
    let prev_item = seq.get(prev_seq_idx).unwrap();
    // Repeat our 2-tuple at least as many times as we need to get running changes between elements
    // in seq
    let seq_changes: Vec<_> = changes.iter().cycle().take(seq.len()).collect();
    // Get the change that *should* happen from element before question mark and question mark
    // Then apply that change to index of previous element (index in LETTERS, not seq) and
    // calculate index (still in LETTERS) of character that appears under question mark
    // Finally, get the character under the index from LETTER and return it
    let change = seq_changes.get(prev_seq_idx).unwrap();
    if let Some(prev_letters_idx) = LETTERS.iter().position(|&n| n == *prev_item) {
        let needle_letters_idx = prev_letters_idx + **change;
        return *LETTERS.get(needle_letters_idx).unwrap();
    }
    '\0'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = missing_letter(&['a', 'c', '?', 'g', 'i']);
        assert_eq!(result, 'e');
    }

    #[test]
    fn test2() {
        let result = missing_letter(&['a', 'd', '?', 'j', 'm']);
        assert_eq!(result, 'g');
    }

    #[test]
    fn test3() {
        let result = missing_letter(&['a', 'e', '?', 'm', 'q']);
        assert_eq!(result, 'i');
    }

    #[test]
    fn test4() {
        let result = missing_letter(&['a', 'c', 'f', '?', 'k']);
        assert_eq!(result, 'h');
    }

    #[test]
    fn test5() {
        let result = missing_letter(&['b', 'e', 'g', '?', 'l']);
        assert_eq!(result, 'j');
    }
}
