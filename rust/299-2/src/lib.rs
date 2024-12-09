use std::collections::HashSet;

fn char_on_position(chars: &[&Vec<char>], position: (usize, usize)) -> Option<char> {
    let (i, j) = position;
    chars.get(i).and_then(|r| r.get(j).copied())
}

fn construct_word(chars: &[&Vec<char>], word: &str, position: (usize, usize), visited: &HashSet<(usize, usize)>) -> bool {
    let needle = word.chars().next().unwrap_or('\0');
    let (i, j) = position;
    let max_i = chars.len();
    let max_j = chars.first().map_or(0, |r| r.len());
    let mut new_visited = visited.clone();
    new_visited.insert(position);

    let new_positions = [
        (i.checked_sub(1), Some(j)),
        (i.checked_add(1), Some(j)),
        (Some(i), j.checked_add(1)),
        (Some(i), j.checked_add(1)),
    ];
    // there's probably more idiomatic way of writing this
    let new_positions = new_positions.iter()
        .filter(|(i, j)| {
            i.is_some_and(|ii| max_i > ii) && j.is_some_and(|jj| max_j > jj)
        })
        .map(|(i, j)| (i.unwrap(), j.unwrap()))
        .filter(|pos| !visited.contains(pos));

    for next_step in new_positions {
        if let Some(letter) = char_on_position(chars, next_step) {
            if letter != needle {
                continue
            }
            if word.chars().count() == 1 {
                return true;
            }
            let new_word = word.get(1..).unwrap();
            return construct_word(chars, new_word, next_step, &new_visited);
        }
    }
    false
}

pub fn word_search(chars: &[&Vec<char>], word: &str) -> bool {
    let needle = word.chars().next().unwrap();
    for (i, row) in chars.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            if *letter != needle {
                continue;
            }
            let new_word = word.get(1..).unwrap();
            let visited: HashSet<(usize, usize)> = [(i, j)].into();
            let can_construct_word = construct_word(chars, new_word, (i, j), &visited);
            if can_construct_word {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let chars = &[
            &vec!('A', 'B', 'D', 'E'),
            &vec!('C', 'B', 'C', 'A'),
            &vec!('B', 'A', 'A', 'D'),
            &vec!('D', 'B', 'B', 'C'),
        ];
        let result = word_search(chars, "BDCA");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let chars = &[
            &vec!('A', 'A', 'B', 'B'),
            &vec!('C', 'C', 'B', 'A'),
            &vec!('C', 'A', 'A', 'A'),
            &vec!('B', 'B', 'B', 'B'),
        ];
        let result = word_search(chars, "ABAC");
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let chars = &[
            &vec!('B', 'A', 'B', 'A'),
            &vec!('C', 'C', 'C', 'C'),
            &vec!('A', 'B', 'A', 'B'),
            &vec!('B', 'B', 'A', 'A'),
        ];
        let result = word_search(chars, "CCCAA");
        assert_eq!(result, true);
    }
}
