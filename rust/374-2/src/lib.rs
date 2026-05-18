use std::cmp::Ordering;

pub fn largest_number(str: &str) -> usize {
    let mut longest_group: Vec<String> = Vec::with_capacity(str.len());
    let mut longest_len: usize = usize::MIN;
    let binding = str.chars().collect::<Vec<_>>();
    let groups = binding.chunk_by(|a, b| a == b);
    for group in groups {
        if group.contains(&'0') {
            continue;
        }
        let group_len = group.len();
        match longest_len.cmp(&group_len) {
            Ordering::Greater => (),
            Ordering::Equal => longest_group.push(group.iter().collect()),
            Ordering::Less => {
                longest_len = group_len;
                longest_group.clear();
                longest_group.push(group.iter().collect());
            }
        }
    }
    longest_group.sort_unstable();
    longest_group.last().unwrap().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = largest_number("6777133339");
        assert_eq!(result, 3333);
    }

    #[test]
    fn test2() {
        let result = largest_number("1200034");
        assert_eq!(result, 4);
    }

    #[test]
    fn test3() {
        let result = largest_number("44221155");
        assert_eq!(result, 55);
    }

    #[test]
    fn test4() {
        let result = largest_number("88888");
        assert_eq!(result, 88888);
    }

    #[test]
    fn test5() {
        let result = largest_number("11122233");
        assert_eq!(result, 222);
    }
}
