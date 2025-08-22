use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Cell {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameResult {
    Winner(char),
    Draw,
    Pending,
}

// That's basically a const, but you can't put function calls into const
fn get_winning_positions() -> [HashSet<Cell>; 8] {
    [
        HashSet::from([
            Cell { x: 0, y: 0 },
            Cell { x: 1, y: 0 },
            Cell { x: 2, y: 0 },
        ]),
        HashSet::from([
            Cell { x: 0, y: 1 },
            Cell { x: 1, y: 1 },
            Cell { x: 2, y: 1 },
        ]),
        HashSet::from([
            Cell { x: 0, y: 2 },
            Cell { x: 1, y: 2 },
            Cell { x: 2, y: 2 },
        ]),
        HashSet::from([
            Cell { x: 0, y: 0 },
            Cell { x: 0, y: 1 },
            Cell { x: 0, y: 2 },
        ]),
        HashSet::from([
            Cell { x: 1, y: 0 },
            Cell { x: 1, y: 1 },
            Cell { x: 1, y: 2 },
        ]),
        HashSet::from([
            Cell { x: 2, y: 0 },
            Cell { x: 2, y: 1 },
            Cell { x: 2, y: 2 },
        ]),
        HashSet::from([
            Cell { x: 0, y: 0 },
            Cell { x: 1, y: 1 },
            Cell { x: 2, y: 2 },
        ]),
        HashSet::from([
            Cell { x: 2, y: 0 },
            Cell { x: 1, y: 1 },
            Cell { x: 0, y: 2 },
        ]),
    ]
}

pub fn find_winner(moves: &[&Cell]) -> GameResult {
    let (a_moves, b_moves): (Vec<_>, Vec<_>) = moves
        .iter()
        .enumerate()
        .partition(|(idx, _e)| idx.rem_euclid(2) == 0);
    let a_moves: HashSet<_> = a_moves.iter().map(|f| **f.1).collect();
    let b_moves: HashSet<_> = b_moves.iter().map(|f| **f.1).collect();

    let winning_positions = get_winning_positions();

    if a_moves.len() >= 3
        && winning_positions
            .iter()
            .any(|winner| winner.is_subset(&a_moves))
    {
        return GameResult::Winner('A');
    }

    if b_moves.len() >= 3
        && winning_positions
            .iter()
            .any(|winner| winner.is_subset(&b_moves))
    {
        return GameResult::Winner('B');
    }

    // The real life is more complex. Apparently, it can be decided that
    // a game will conclude in a draw after 7 moves, but not all 7 moves
    // board states are draws.
    // To check if a game is a draw or not you are supposed to try each
    // symbol in each empty space and check if that would be a win -
    // after exhausting all the options without finding a possible win you
    // can conclude this is a guaranteed draw.
    // I don't care nearly enough, so I'm going to call it a draw only
    // when board is filled completely and we haven't found a winner.
    if moves.len() == 9 {
        GameResult::Draw
    } else {
        GameResult::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = find_winner(&[
            &Cell { x: 0, y: 0 },
            &Cell { x: 2, y: 0 },
            &Cell { x: 1, y: 1 },
            &Cell { x: 2, y: 1 },
            &Cell { x: 2, y: 2 },
        ]);
        assert_eq!(result, GameResult::Winner('A'));
    }

    #[test]
    fn test2() {
        let result = find_winner(&[
            &Cell { x: 0, y: 0 },
            &Cell { x: 1, y: 1 },
            &Cell { x: 0, y: 1 },
            &Cell { x: 0, y: 2 },
            &Cell { x: 1, y: 0 },
            &Cell { x: 2, y: 0 },
        ]);
        assert_eq!(result, GameResult::Winner('B'));
    }

    #[test]
    fn test3() {
        let result = find_winner(&[
            &Cell { x: 0, y: 0 },
            &Cell { x: 1, y: 1 },
            &Cell { x: 2, y: 0 },
            &Cell { x: 1, y: 0 },
            &Cell { x: 1, y: 2 },
            &Cell { x: 2, y: 1 },
            &Cell { x: 0, y: 1 },
            &Cell { x: 0, y: 2 },
            &Cell { x: 2, y: 2 },
        ]);
        assert_eq!(result, GameResult::Draw);
    }

    #[test]
    fn test4() {
        let result = find_winner(&[&Cell { x: 0, y: 0 }, &Cell { x: 1, y: 1 }]);
        assert_eq!(result, GameResult::Pending);
    }

    #[test]
    fn test5() {
        let result = find_winner(&[
            &Cell { x: 1, y: 1 },
            &Cell { x: 0, y: 0 },
            &Cell { x: 2, y: 2 },
            &Cell { x: 0, y: 1 },
            &Cell { x: 1, y: 0 },
            &Cell { x: 0, y: 2 },
        ]);
        assert_eq!(result, GameResult::Winner('B'));
    }
}
