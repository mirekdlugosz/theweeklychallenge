use std::collections::HashSet;

#[derive(Debug)]
pub struct DominoPiece {
    a: usize,
    b: usize,
}

impl PartialEq for DominoPiece {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

pub fn similar_dominos(pieces: &[DominoPiece]) -> usize {
    let mut similar: HashSet<usize> = HashSet::new();

    for (i, piece_i) in pieces.iter().enumerate() {
        for (j, piece_j) in pieces.iter().enumerate().skip(i + 1) {
            if piece_i == piece_j {
                similar.insert(i);
                similar.insert(j);
            }
        }
    }
    similar.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let pieces = [
            DominoPiece { a: 1, b: 3 },
            DominoPiece { a: 3, b: 1 },
            DominoPiece { a: 2, b: 4 },
            DominoPiece { a: 6, b: 8 },
        ];

        let result = similar_dominos(&pieces);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let pieces = [
            DominoPiece { a: 1, b: 2 },
            DominoPiece { a: 2, b: 1 },
            DominoPiece { a: 1, b: 1 },
            DominoPiece { a: 1, b: 2 },
            DominoPiece { a: 2, b: 2 },
        ];

        let result = similar_dominos(&pieces);
        assert_eq!(result, 3);
    }
}
