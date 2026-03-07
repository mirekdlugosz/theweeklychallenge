use std::net::Ipv4Addr;

pub fn subnet_sheriff(ip_addr: &str, domain: &str) -> bool {
    let parsed_ip: Ipv4Addr = match ip_addr.parse() {
        Ok(addr) => addr,
        Err(_) => return false,
    };
    let (domain_ip, domain_mask) = domain.rsplit_once('/').unwrap();
    let parsed_domain_mask: u32 = match domain_mask.parse() {
        Ok(mask) => mask,
        Err(_) => return false,
    };
    let parsed_domain_ip: Ipv4Addr = match domain_ip.parse() {
        Ok(addr) => addr,
        Err(_) => return false,
    };

    let bitmask = u32::MAX - 1 << (32 - parsed_domain_mask);
    parsed_ip.to_bits() & bitmask == parsed_domain_ip.to_bits() & bitmask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = subnet_sheriff("192.168.1.45", "192.168.1.0/24");
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let result = subnet_sheriff("10.0.0.256", "10.0.0.0/24");
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let result = subnet_sheriff("172.16.8.9", "172.16.8.9/32");
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let result = subnet_sheriff("172.16.4.5", "172.16.0.0/14");
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = subnet_sheriff("192.0.2.0", "192.0.2.0/25");
        assert_eq!(result, true);
    }
}
