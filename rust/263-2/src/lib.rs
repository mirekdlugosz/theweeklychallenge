use std::collections::HashMap;

type Matrix2D = Vec<Vec<usize>>;

pub fn merge_items(items1: &Matrix2D, items2: &Matrix2D) -> Matrix2D {
    let mut storage: HashMap<usize, usize> = HashMap::new();
    let mut returned_storage: Matrix2D = Vec::with_capacity(items1.len() + items2.len());

    let combined = items1.iter().chain(items2);

    for item_spec in combined {
        let [item_id, item_number] = item_spec[..] else { panic!("Invalid input") };
        storage
            .entry(item_id)
            .and_modify(|counter| *counter += item_number)
            .or_insert(item_number);
    }

    for (key, value) in storage.iter() {
        returned_storage.push(vec!(*key, *value));
    }

    returned_storage.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

    returned_storage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let items1 = vec!(
            vec!(1,1), vec!(2,1), vec!(3,2)
        );
        let items2 = vec!(
            vec!(2,2), vec!(1,3)
        );
        let result = merge_items(&items1, &items2);
        let expected = vec!(
            vec!(1,4), vec!(2,3), vec!(3,2)
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let items1 = vec!(
            vec!(1,2), vec!(2,3), vec!(1,3), vec!(3,2)
        );
        let items2 = vec!(
            vec!(3,1), vec!(1,3)
        );
        let result = merge_items(&items1, &items2);
        let expected = vec!(
            vec!(1,8), vec!(2,3), vec!(3,3)
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let items1 = vec!(
            vec!(1,1), vec!(2,2), vec!(3,3)
        );
        let items2 = vec!(
            vec!(2,3), vec!(2,4)
        );
        let result = merge_items(&items1, &items2);
        let expected = vec!(
            vec!(1,1), vec!(2,9), vec!(3,3)
        );
        assert_eq!(result, expected);
    }
}
