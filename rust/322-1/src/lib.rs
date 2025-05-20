use std::iter::{once, repeat_n};

pub fn string_format(input: &str, i: usize) -> String {
    let letters_num = input.chars().filter(|&c| c != '-').count();
    let first_group = letters_num.rem_euclid(i);
    let full_groups_num = letters_num.div_euclid(i);
    let group_iter = once(first_group).chain(repeat_n(i, full_groups_num));
    let mut output = String::new();
    let mut chars = input.chars().filter(|&c| c != '-');

    for group_size in group_iter {
        for ch in chars.by_ref().take(group_size) {
            output.push(ch);
        }
        output.push('-');
    }

    output.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = string_format("ABC-D-E-F", 3);
        assert_eq!(result, "ABC-DEF");
    }

    #[test]
    fn test2() {
        let result = string_format("A-BC-D-E", 2);
        assert_eq!(result, "A-BC-DE");
    }

    #[test]
    fn test3() {
        let result = string_format("-A-B-CD-E", 4);
        assert_eq!(result, "A-BCDE");
    }
}
