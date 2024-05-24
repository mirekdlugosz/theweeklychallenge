pub fn defang(ip_addr: &str) -> String {
    ip_addr
        .split('.')
        .collect::<Vec<&str>>()
        .join("[.]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = String::from("1[.]1[.]1[.]1");
        assert_eq!(defang("1.1.1.1"), result);
    }

    #[test]
    fn test2() {
        let result = String::from("255[.]101[.]1[.]0");
        assert_eq!(defang("255.101.1.0"), result);
    }
}
