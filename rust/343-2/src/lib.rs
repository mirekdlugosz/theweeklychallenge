type Grid = Vec<Vec<usize>>;

fn idx_of_max(grid: &Grid) -> Vec<usize> {
    let counts: Vec<_> = grid.iter().map(|r| r.iter().sum()).collect();
    let max = counts.iter().max().unwrap_or(&usize::MAX);
    counts
        .iter()
        .enumerate()
        .filter_map(|(idx, count)| if count == max { Some(idx) } else { None })
        .collect()
}

// The easy case is easy - just count the number of wins in each row, return
// the index of row with highest sum.
// The problem starts when we have a tie.
// Here, we create a new grid that only contains the information about teams
// with a tie, and look for the team that has the highest number of wins. That
// is the team that won against other best teams.
// We can do it because task description allows us to assume there is always
// a clear winner. Technically, there could be cycles - e.g. team A wins against
// team B, team B wins against team C and team C wins against team A. Everyone
// has a single win, so there is a tie, but there is no clear ordering.

pub fn champion_team(grid: &Grid) -> usize {
    let teams = idx_of_max(grid);
    if teams.len() == 1 {
        return *teams.first().unwrap();
    }
    let new_grid: Grid = grid
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if teams.contains(&idx) {
                let mut new_row = Vec::with_capacity(row.len());
                for elem in &teams {
                    if let Some(val) = row.get(*elem) {
                        new_row.push(*val);
                    }
                }
                Some(new_row)
            } else {
                None
            }
        })
        .collect();
    let new_teams = idx_of_max(&new_grid);
    let champion_idx = *new_teams.first().unwrap_or(&usize::MAX);
    *teams.get(champion_idx).unwrap_or(&usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = champion_team(&vec![vec![0, 1, 1], vec![0, 0, 1], vec![0, 0, 0]]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test2() {
        let result = champion_team(&vec![
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
            vec![1, 1, 0, 0],
            vec![1, 1, 1, 0],
        ]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let result = champion_team(&vec![
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 1],
            vec![1, 0, 0, 0],
            vec![0, 0, 1, 0],
        ]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let result = champion_team(&vec![vec![0, 1, 1], vec![0, 0, 0], vec![0, 1, 0]]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test5() {
        let result = champion_team(&vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 1, 1],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 0, 1, 0],
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test6() {
        let result = champion_team(&vec![
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 0, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 0, 0, 0],
        ]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test7() {
        let result = champion_team(&vec![
            vec![0, 1, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0],
            vec![1, 0, 0, 1, 1, 1, 0], // 4 wins, but better
            vec![0, 0, 0, 0, 1, 1, 1],
            vec![1, 1, 0, 0, 0, 1, 1], // 4 wins
            vec![1, 1, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 0, 0],
        ]);
        assert_eq!(result, 2);
    }
}
