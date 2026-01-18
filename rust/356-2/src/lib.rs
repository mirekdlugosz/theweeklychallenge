fn get_winner(result: char, teams: (u8, u8)) -> u8 {
    match result {
        'H' => teams.0,
        'A' => teams.1,
        _ => 0
    }
}

pub fn who_wins(results: &str) -> String {
    let mut winners: Vec<u8> = Vec::with_capacity(4);
    // week 1
    let placement: std::slice::Iter<'_, (u8, u8)> = [(2, 7), (3, 6), (4, 5)].iter();
    for (result, teams) in results.chars().take(3).zip(placement) {
        winners.push(get_winner(result, *teams));
    }
    // week 2
    let week2A = winners.iter().copied().max().unwrap_or(0);
    let mut week2others: Vec<u8> = winners.iter().filter(|e| **e != week2A).copied().collect();
    week2others.sort_unstable();
    let week2others: (u8, u8) = (*week2others.first().unwrap_or(&0), *week2others.last().unwrap_or(&0));
    winners.clear();
    winners.push(get_winner(results.chars().nth(3).unwrap_or('\0'), (1, week2A)));
    winners.push(get_winner(results.chars().nth(4).unwrap_or('\0'), week2others));

    // week 3 and last
    winners.sort_unstable();

    let teamA = winners.first().unwrap_or(&0);
    let teamB = winners.last().unwrap_or(&0);

    let winner = get_winner(results.chars().nth(5).unwrap_or('\0'), (*teamA, *teamB));
    let loser = if winner == *teamA { *teamB } else { *teamA };

    format!("Team {winner} defeated Team {loser}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = who_wins("HAHAHH");
        assert_eq!(result, "Team 2 defeated Team 6");
    }

    #[test]
    fn test2() {
        let result = who_wins("HHHHHH");
        assert_eq!(result, "Team 1 defeated Team 2");
    }

    #[test]
    fn test3() {
        let result = who_wins("HHHAHA");
        assert_eq!(result, "Team 4 defeated Team 2");
    }

    #[test]
    fn test4() {
        let result = who_wins("HAHAAH");
        assert_eq!(result, "Team 4 defeated Team 6");
    }

    #[test]
    fn test5() {
        let result = who_wins("HAAHAA");
        assert_eq!(result, "Team 5 defeated Team 1");
    }
}
