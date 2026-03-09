pub fn goal_parser(str: &str) -> String {
    str.replace("()", "o").replace("(al)", "al")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = goal_parser("G()(al)");
        assert_eq!(result, "Goal");
    }

    #[test]
    fn test2() {
        let result = goal_parser("G()()()()(al)");
        assert_eq!(result, "Gooooal");
    }

    #[test]
    fn test3() {
        let result = goal_parser("(al)G(al)()()");
        assert_eq!(result, "alGaloo");
    }

    #[test]
    fn test4() {
        let result = goal_parser("()G()G");
        assert_eq!(result, "oGoG");
    }

    #[test]
    fn test5() {
        let result = goal_parser("(al)(al)G()()");
        assert_eq!(result, "alalGoo");
    }
}
