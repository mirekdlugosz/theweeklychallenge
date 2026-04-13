pub fn valid_tag(title: &str) -> String {
    let tag_itself = title
        .split_whitespace()
        .enumerate()
        .map(|(idx, word)| {
            word.chars()
                .filter(char::is_ascii_alphabetic)
                .enumerate()
                .map(|(cidx, ch)| match (idx, cidx) {
                    (0, 0) => ch.to_ascii_lowercase(),
                    (_, 0) => ch.to_ascii_uppercase(),
                    (_, _) => ch.to_ascii_lowercase(),
                })
                .collect::<String>()
        })
        .collect::<String>()
        .chars()
        .take(99)
        .collect::<String>();
    format!("#{tag_itself}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = valid_tag("Cooking with 5 ingredients!");
        assert_eq!(result, "#cookingWithIngredients");
    }

    #[test]
    fn test2() {
        let result = valid_tag("the-last-of-the-mohicans");
        assert_eq!(result, "#thelastofthemohicans");
    }

    #[test]
    fn test3() {
        let result = valid_tag("  extra spaces here");
        assert_eq!(result, "#extraSpacesHere");
    }

    #[test]
    fn test4() {
        let result = valid_tag("iPhone 15 Pro Max Review");
        assert_eq!(result, "#iphoneProMaxReview");
    }

    #[test]
    fn test5() {
        let result = valid_tag(
            "Ultimate 24-Hour Challenge: Living in a Smart Home controlled entirely by Artificial Intelligence and Voice Commands in the year 2026!",
        );
        assert_eq!(
            result,
            "#ultimateHourChallengeLivingInASmartHomeControlledEntirelyByArtificialIntelligenceAndVoiceCommandsIn"
        );
    }
}
