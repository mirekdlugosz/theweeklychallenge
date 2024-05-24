use std::collections::HashMap;

pub fn sort_by_bits(ints: &[usize]) -> Vec<usize> {
    let mut hash: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut output: Vec<usize> = Vec::with_capacity(ints.len());

    for i in ints.iter() {
        let bits_num = i.count_ones();
        hash.entry(bits_num)
            .or_default()
            .push(*i);
    }

    let mut sorted_ones: Vec<&u32> = hash.keys().collect();
    sorted_ones.sort_unstable();

    for bits_num in sorted_ones {
        if let Some(v) = hash.get(bits_num) {
            let mut values: Vec<usize> = v.clone();
            values.sort_unstable();
            output.extend(values);
        }
    } 
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ints = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8);
        let expected = vec!(0, 1, 2, 4, 8, 3, 5, 6, 7);
            
        assert_eq!(sort_by_bits(&ints), expected);
    }

    #[test]
    fn test2() {
        let ints = vec!(1024, 512, 256, 128, 64);
        let expected = vec!(64, 128, 256, 512, 1024);
            
        assert_eq!(sort_by_bits(&ints), expected);
    }
}
