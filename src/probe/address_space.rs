use std::{net::Ipv4Addr, ops::Add};

use regex::Regex;

pub struct NameError{}

pub struct AddressSpaceIpv4{
    start: Ipv4Addr,
    end: Ipv4Addr,
    curr: Ipv4Addr,
    next: Ipv4Addr
}

impl Iterator for AddressSpaceIpv4 {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Self::Item> {
        if current.octet(0) >= 255 {
            return None
        }
        if current.octet(1) >= 255 {
            self.next = Ipv4Addr::new(current.octet(1) + 1, 0, 0, 0);
        }
        if current.octet(2) >= 255 {
            self.next = Ipv4Addr::new(current.octet(1), current.octet(2) + 1, 0, 0);
        }
        if current.octet(3) >= 255 {
            self.next = Ipv4Addr::new(current.octet(0), current.octet(1), current.octet())
        }
        return Some(Ipv4Addr::new(192, 168, 0, 1))
    }
}

pub struct AddressSpace {}
impl AddressSpace {
    fn validate_dns(s: &str) -> Result<(), NameError> {
        if s.len() > 253 || s.len() <= 0 || s.contains(" ") || s.starts_with("-") {
            return Err(NameError{})
        }
        let splitted_name = s.split('.');
        for i in splitted_name{
            if i.starts_with("-") || i.ends_with("-"){
                return Err(NameError{})
            }
            if i.is_empty() {
                return Err(NameError{})
            }
            let re = Regex::new(r"^([A-Za-z0-9-]{1, 63})+$").unwrap();
            match re.is_match(i) {
                true => (),
                false => return Err(NameError{})
            }
        }
        Ok(())
    }
    fn ipv4_range(start: Ipv4Addr, end: Ipv4Addr) -> AddressSpaceIpv4 {
        return AddressSpaceIpv4{}
    }
}
#[cfg(test)]
pub mod as_test{
    use std::net::Ipv4Addr;

    use super::AddressSpace;
    #[test]
    fn valid_names_return_ok(){
        let names = vec!["TEST", 
                                    "TEST01", 
                                    "TEST1234",
                                    "123TEST",
                                    "test", 
                                    "test01", 
                                    "test1234", 
                                    "test.verdete.com", 
                                    "test01.verdete.com", 
                                    "test123123.verdete.com",
                                    "test-123-123.verdete.com",
                                    "abcdefghijklmnopqrstuvwxyz-1234567890.verdete.com"
                                    ];
        for i in names {
            assert!(AddressSpace::validate_dns(i).err().is_none());
        }
    }

    #[test]
    fn invalid_names_return_error(){
        let names = vec!["!nv@l!d",
                                    "TEST...",
                                    "@@TEST.",
                                    "-TEST.verdete.com",
                                    "TEST-.verdete.com",
                                    "",
                                    "<HELLO>",
                                    r"<\",
                                    "<",
                                    ">",
                                    "</",
                                    "\n",
                                    "\r",
                                    " ",
                                    r"\",
                                    r"/"
                                    ];
        for i in names {
            assert!(!(AddressSpace::validate_dns(i).err().is_none()));
        }
    }

    #[test]
    fn ipv4_range_iteration_is_ok(){
        let range = AddressSpace::ipv4_range(Ipv4Addr::new(192, 168, 0, 0), Ipv4Addr::new(192, 168, 0, 1));
        let fourth_octet = 0;
        for i in range {
            assert!(i.octet(3) > fourth_octet);
            fourth_octet = i.octet(3);
        }

        let range = AddressSpace::ipv4_range(Ipv4Addr::new(192, 168, 0, 0), Ipv4Addr::new(192, 168, 10, 0));
        let third_octet = 0;
        let fourth_octet = 0;
        for i in range {
            if i.octet(3) <= fourth_octet {
                assert!(i.octet(2) > third_octet)
            } else {
                assert!(i.octet(3) > fourth_octet);
            }
            third_octet = i.octet(2);
            fourth_octet = i.octet(3);
        }
    }

    #[test]
    fn ipv6_range_iteration_is_ok(){}

    #[test]
    fn valid_ipv4_return_ok(){}

    #[test]
    fn invalid_ipv4_return_error(){}

    #[test]
    fn valid_ipv6_return_ok(){}

    #[test]
    fn invalid_ipv6_return_error(){}

    #[test]
    fn address_space_construction_ok(){}

    #[test]
    fn ldap_as_loading_is_ok(){}

    #[test]
    fn json_as_loading_is_ok(){}

    #[test]
    fn yaml_as_loading_is_ok(){}

    #[test]
    fn toml_as_loading_is_ok(){}

    #[test]
    fn csv_as_loading_is_ok(){}

    #[test]
    fn psql_as_loading_is_ok(){}

    #[test]
    fn mysql_as_loading_is_ok(){}

    #[test]
    fn redis_as_loading_is_ok(){}

    #[test]
    fn boltdb_as_loading_is_ok(){}

    #[test]
    fn mongo_as_loading_is_ok(){}

    #[test]
    fn influx_as_loading_is_ok(){}

    #[test]
    fn probe_as_loading_is_ok(){}
}