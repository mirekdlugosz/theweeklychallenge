use std::collections::{HashMap, HashSet};


pub fn unique_occurences(ints: &[isize]) -> usize {
    let mut counter: HashMap<isize, usize> = HashMap::new();
    let mut seen: HashSet<usize> = HashSet::new();
    for int in ints {
        counter.entry(*int).and_modify(|i| *i += 1).or_insert(1);
    }
    for value in counter.values() {
        match seen.contains(value) {
            true => return 0,
            false => seen.insert(*value),
        };
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![1,2,2,1,1,3];
        assert_eq!(unique_occurences(&v), 1);
    }

    #[test]
    fn test2() {
        let v = vec![1,2,3];
        assert_eq!(unique_occurences(&v), 0);
    }

    #[test]
    fn test3() {
        let v = vec![-2,0,1,-2,1,1,0,1,-2,9];
        assert_eq!(unique_occurences(&v), 1);
    }
}
