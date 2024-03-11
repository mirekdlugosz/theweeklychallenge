use itertools::Itertools;

pub fn rank(word: &str) -> usize {
    let mut ranking: Vec<String> = Vec::new();
    for permutation in word.chars().permutations(word.len()) {
        let new_word = permutation.iter().join("");
        ranking.push(new_word);
    }
    ranking.sort_unstable();
    let rv = ranking.iter().dedup().position(|new_word| new_word == word).unwrap();
    rv + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(rank("CAT"), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(rank("GOOGLE"), 88);
    }

    #[test]
    fn test3() {
        assert_eq!(rank("SECRET"), 255);
    }
}
