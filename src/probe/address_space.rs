use std::{net::{Ipv4Addr, Ipv6Addr}, ops::Add, slice::Iter, str::FromStr, u8};

use ldap3::{LdapConn, SearchEntry};
use regex::Regex;
use serde_json::{Error, Value};
use serde_json_path::JsonPath;

#[derive(Debug)]
pub struct AddrError{}

#[derive(Debug)]
pub struct AddressSpaceError{}

pub trait AddressSpace{}

#[derive(Debug)]
pub struct AddressSpaceIpv6Range {
    start: Ipv6Addr,
    end: Ipv6Addr,
    curr: Ipv6Addr,
    next: Ipv6Addr
}

impl AddressSpaceIpv6Range {
    fn new(start: Ipv6Addr, end: Ipv6Addr) -> AddressSpaceIpv6Range {
        return AddressSpaceIpv6Range {
            start,
            end,
            curr: start,
            next: start
        }
    }
}

impl Iterator for AddressSpaceIpv6Range {
    type Item = Ipv6Addr;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = self.next;
        if self.curr > self.end {
            return None
        }
        self.next = Ipv6Addr::from_bits(self.curr.to_bits() + 1);
        return Some(self.curr)
    }
}

impl AddressSpace for AddressSpaceIpv6Range{}

#[derive(Debug)]
pub struct AddressSpaceIpv4Range{
    start: Ipv4Addr,
    end: Ipv4Addr,
    curr: Ipv4Addr,
    next: Ipv4Addr
}

impl AddressSpaceIpv4Range {
    fn new(start: Ipv4Addr, end: Ipv4Addr) -> AddressSpaceIpv4Range {
        return AddressSpaceIpv4Range {
            start,
            end,
            curr: start,
            next: start
        }
    }
}

impl Iterator for AddressSpaceIpv4Range {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = self.next;
        if self.curr > self.end {
            return None
        }
        self.next = Ipv4Addr::from_bits(self.curr.to_bits() + 1);
        return Some(self.curr)
    }
}

impl AddressSpace for AddressSpaceIpv4Range{}

#[derive(Debug)]
pub struct AddressSpaceAddrList {
    pub addrs: Vec<Address>,
}

impl AddressSpaceAddrList {
    fn new(addrs: Vec<Address>) -> AddressSpaceAddrList {
        return AddressSpaceAddrList {
            addrs
        }
    }

    fn iter(&self) -> Iter<Address> {
        return self.addrs.iter()
    }

    fn contains(&self, addr: &Address) -> bool {
        return self.addrs.contains(addr);
    }

    fn len(&self) -> usize {
        return self.addrs.len()
    }
}

impl AddressSpace for AddressSpaceAddrList{}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressType {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
    DNS(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    addr_type: AddressType
}

impl Address {
    fn validate_dns(s: &str) -> Result<(), AddrError> {
        if s.len() > 253 || s.len() <= 0 || s.contains(" ") || s.starts_with("-") {
            return Err(AddrError{})
        }
        let splitted_name = s.split('.');
        for i in splitted_name{
            if i.starts_with("-") || i.ends_with("-"){
                return Err(AddrError{})
            }
            if i.is_empty() {
                return Err(AddrError{})
            }
            let re = Regex::new(r"^([A-Za-z0-9-]{1, 63})+$").unwrap();
            match re.is_match(i) {
                true => (),
                false => return Err(AddrError{})
            }
        }
        Ok(())
    }
    
    pub fn ipv4_from_str(s: &str) -> Result<Address, AddrError> {
        match Ipv4Addr::from_str(s) {
            Ok(addr) => return Ok(Address { addr_type: AddressType::Ipv4(addr) }),
            Err(err) => return Err(AddrError { })
        }
    }

    pub fn ipv6_from_str(s: &str) -> Result<Address, AddrError> {
        match Ipv6Addr::from_str(s) {
            Ok(addr) => return Ok(Address { addr_type: AddressType::Ipv6(addr) }),
            Err(err) => return Err(AddrError { })
        }
    }

    pub fn dns_from_str(s: &str) -> Result<Address, AddrError> {
        match Address::validate_dns(s) {
            Ok(()) => return Ok(Address{ addr_type: AddressType::DNS(s.to_string()) }),
            Err(err) => return Err(err)
        }
    }

    pub fn from_str(s: &str) -> Result<Address, AddrError> {
        if s.contains(":") {
            match Address::ipv6_from_str(s) {
                Ok(ipv6) => return Ok(ipv6),
                Err(err) => return Err(AddrError { })
            }
        } else {
            match Address::ipv4_from_str(s) {
                Ok(addr) => return Ok(addr),
                Err(_) => match Address::dns_from_str(s) {
                    Ok(addr) => return Ok(addr),
                    Err(e) => return Err(e)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct AddressSpaceFactory {}

impl AddressSpaceFactory{
    pub fn from_json(json_string: &str, selector: &str) -> Result<AddressSpaceAddrList, AddressSpaceError>{
        if selector.len() == 0 {
            return Err(AddressSpaceError {  })
        }
        let parsed: Value = serde_json::from_str(json_string).unwrap();
        let selector = JsonPath::parse(selector).unwrap();
        let query = selector.query(&parsed);
        let results = query.all();
        let mut parsed_addrs: Vec<Address> = Vec::new();
        if results.len() <= 0 {
            return Err(AddressSpaceError { })
        } else {
            for i in results {
                if !(i.is_string()) {
                    return Err(AddressSpaceError { })
                }
                parsed_addrs.push(Address::from_str(i.as_str().unwrap()).unwrap());
            }
        }
        Ok(AddressSpaceAddrList::new(parsed_addrs))
    }
    
    /*pub fn from_yaml<T>(yaml_string: &str, selector: &str) -> Result<T, AddressSpaceError> where T: AddressSpace {}

    pub fn from_toml<T>(toml_string: &str, selector: &str) -> Result<T, AddressSpaceError> where T: AddressSpace {}

    pub fn from_csv<T>(csv_string: &str, column: &str, sep: Option<&str>) -> Result<T, AddressSpaceError> where T: AddressSpace {}

    pub fn from_sql<T>(dialect: &str, query_string: &str, connection_string: &str) -> Result<T, AddressSpaceError> where T: AddressSpace {}

    pub fn from_mongo<T>(selection: Option<&str>, projection: Option<&str>, connection_string: Option<&str>) -> Result<T, AddressSpaceError> where T: AddressSpace {}
*/
    pub fn from_ldap(srv_addr: &str, 
                     srv_port: Option<u16>, 
                     bind_dn: Option<&str>, 
                     bind_dn_password: Option<&str>, 
                     base_dn: &str, 
                     ldap_filter: &str, 
                     use_ldaps: bool) -> Result<AddressSpaceAddrList, AddrError>{
        let mut ldap = match use_ldaps {
            true => LdapConn::new(format!("ldaps://{srv_addr}:{}", srv_port.unwrap_or(636)).as_str()).unwrap(),
            false => LdapConn::new(format!("ldap://{srv_addr}:{}", srv_port.unwrap_or(389)).as_str()).unwrap()
        };
        if let Some(dn) = bind_dn {
            if let Some(pw) = bind_dn_password{
                ldap.simple_bind(dn, pw).unwrap();
            }
        }
        let (rs, _res) = ldap.search(
            &base_dn,
            ldap3::Scope::Subtree,
            &ldap_filter,
            &["DnsHostName"]
        ).unwrap().success().unwrap();
        let mut valid_names: Vec<Address> = Vec::new();
        for entry in rs {
            let se = SearchEntry::construct(entry).attrs["dNSHostName"][0].clone();
            match Address::validate_dns(se.as_str()){
                Ok(()) => valid_names.push(Address::from_str(se.as_str()).unwrap()),
                Err(a) => return Err(a)
            };
        }
        ldap.unbind().unwrap();
        Ok(AddressSpaceAddrList::new(valid_names))
    }
    pub fn ipv4_range(start: Ipv4Addr, end: Ipv4Addr) -> AddressSpaceIpv4Range {
        return AddressSpaceIpv4Range::new(start, end)
    }
    pub fn ipv6_range(start: Ipv6Addr, end: Ipv6Addr) -> AddressSpaceIpv6Range {
        return AddressSpaceIpv6Range::new(start, end)
    }
}
#[cfg(test)]
pub mod as_test{
    use std::net::{Ipv4Addr, Ipv6Addr};

    use crate::probe::address_space::{Address, AddressSpaceAddrList, AddressType};

    use super::{AddressSpace, AddressSpaceFactory};
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
            assert!(Address::validate_dns(i).err().is_none());
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
            assert!(!(Address::validate_dns(i).err().is_none()));
        }
    }

    #[test]
    fn ipv4_list_from_json_is_ok() {
        let json_string = std::fs::read_to_string("/etc/verdete/json_ipv4_list.json").unwrap();
        let ipv4_list = AddressSpaceFactory::from_json(json_string.as_ref(), "$.def.hosts").unwrap();
        assert!(ipv4_list.iter().len() == 10);
        assert!(ipv4_list.contains(&Address::from_str("192.168.0.1").unwrap()));
        assert!(ipv4_list.contains(&Address::from_str("192.168.0.2").unwrap()));
        assert!(ipv4_list.contains(&Address::from_str("192.168.0.5").unwrap()));
        for i in ipv4_list.iter() {
            assert!(matches!(i.addr_type, AddressType::Ipv4(_)))
        }
    }

    #[test]
    fn ipv6_list_from_json_is_ok() {
        let json_string = std::fs::read_to_string("/etc/verdete/json_ipv6_list.json").unwrap();
        let ipv6_list = AddressSpaceFactory::from_json(json_string.as_ref(), "def.hosts").unwrap();
        assert!(ipv6_list.len() == 10);
    }

    #[test]
    fn dns_list_from_json_is_ok() {
        let json_string = std::fs::read_to_string("/etc/verdete/json_dns_list.json").unwrap();
        let dns_list = AddressSpaceFactory::from_json(json_string.as_ref(), "def.hosts").unwrap();
        assert!(dns_list.iter().len() == 10);
        /*for i in dns_list.iter() {
            assert!(Address::validate_dns(i.as_str()).err().is_none());
        }*/
    }

    #[test]
    fn ipv4_range_iteration_is_ok(){
        let range = AddressSpaceFactory::ipv4_range(Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 255));
        let mut fourth_octet = 0;
        for i in range {
            println!("{:?} - {:?} - {:?}", i, i.octets()[3], fourth_octet);
            assert!(i.octets()[3] > fourth_octet);
            fourth_octet = i.octets()[3];
        }

        let range = AddressSpaceFactory::ipv4_range(Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 10, 0));
        let mut third_octet = 0;
        let mut fourth_octet = 0;
        for i in range {
            if i.octets()[3] < fourth_octet {
                assert!(i.octets()[2] > third_octet)
            } else {
                assert!(i.octets()[3] > fourth_octet);
            }
            third_octet = i.octets()[2];
            fourth_octet = i.octets()[3];
        }
    }

    #[test]
    fn ipv6_range_iteration_is_ok(){
        let range = AddressSpaceFactory::ipv6_range(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0xffff));
        let mut last = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
        for i in range {
            assert!(i > last);
            last = i;
        }

        let range = AddressSpaceFactory::ipv6_range(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0x1111, 0xffff));
        let mut last = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
        for i in range {
            assert!(i > last);
            last = i;
        }
    }

    #[test]
    fn ldap_as_loading_is_ok(){
        let names = AddressSpaceFactory::from_ldap("SRVFUVS24414", 
                                                        None,
                                                         Some("CN=VerdeteTest,OU=SISTEMAS,OU=INFORMATICA,OU=_UNIDADE CENTRAL,DC=fuvs,DC=br"),
                                                Some("123456"),
                                                         "OU=COMPUTADORES INFORMATICA,OU=INFORMATICA,OU=_UNIDADE CENTRAL,DC=fuvs,DC=br", 
                                                     "(objectClass=computer)",
                                                       false).unwrap();
        assert!(names.iter().len() > 0);
    }
}