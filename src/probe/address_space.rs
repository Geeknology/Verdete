use std::{net::{Ipv4Addr, Ipv6Addr}, ops::Add};

use ldap3::{LdapConn, SearchEntry};
use regex::Regex;

#[derive(Debug)]
pub struct NameError{}

pub trait AddressSpace{}

#[derive(Debug)]
pub struct AddressSpaceIpv6 {
    start: Ipv6Addr,
    end: Ipv6Addr,
    curr: Ipv6Addr,
    next: Ipv6Addr
}

impl AddressSpaceIpv6 {
    fn new(start: Ipv6Addr, end: Ipv6Addr) -> AddressSpaceIpv6 {
        return AddressSpaceIpv6 {
            start,
            end,
            curr: start,
            next: start
        }
    }
}

impl Iterator for AddressSpaceIpv6 {
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

impl AddressSpace for AddressSpaceIpv6{}

#[derive(Debug)]
pub struct AddressSpaceIpv4{
    start: Ipv4Addr,
    end: Ipv4Addr,
    curr: Ipv4Addr,
    next: Ipv4Addr
}

impl AddressSpaceIpv4 {
    fn new(start: Ipv4Addr, end: Ipv4Addr) -> AddressSpaceIpv4 {
        return AddressSpaceIpv4 {
            start,
            end,
            curr: start,
            next: start
        }
    }
}

impl Iterator for AddressSpaceIpv4 {
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

impl AddressSpace for AddressSpaceIpv4{}

#[derive(Debug)]
pub struct AddressSpaceDNS {
    pub names: Vec<String>,
}

impl AddressSpaceDNS {
    fn new(names: Vec<String>) -> AddressSpaceDNS {
        return AddressSpaceDNS {
            names
        }
    }
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
}

impl AddressSpace for AddressSpaceDNS{}

#[derive(Debug)]
pub struct AddressSpaceFactory {}

impl AddressSpaceFactory {
    pub fn dns(names: Vec<String>) -> Result<AddressSpaceDNS, NameError> {
        let mut valid_names = Vec::new();
        for i in names {
            match AddressSpaceDNS::validate_dns(i.as_str()) {
                Ok(()) => valid_names.push(i),
                Err(a) => return Err(a)
            }
        }
        return Ok(AddressSpaceDNS::new(valid_names));
    }
    pub fn dns_from_json(json_string: &str) -> Result<AddressSpaceDNS, NameError>{
        let parsed: Vec<String> = serde_json::from_str(json_string).unwrap();
        let mut valid_names: Vec<String> = Vec::new();
        for i in parsed {
            match AddressSpaceDNS::validate_dns(i.as_str()) {
                Ok(()) => valid_names.push(i),
                Err(a) => return Err(a)
            }
        }
        return Ok(AddressSpaceDNS { names: valid_names })
    }
    pub fn dns_from_json_file(path: &str) -> Result<AddressSpaceDNS, NameError> {
        let json_string = std::fs::read_to_string(path).unwrap();
        match AddressSpaceFactory::dns_from_json(json_string.as_str()) {
            Ok(addr) => return Ok(addr),
            Err(a) => return Err(a)
        }
    }
    pub fn dns_from_ldap(srv_addr: &str, 
                     srv_port: Option<u16>, 
                     bind_dn: Option<&str>, 
                     bind_dn_password: Option<&str>, 
                     base_dn: &str, 
                     ldap_filter: &str, 
                     use_ldaps: bool) -> Result<AddressSpaceDNS, NameError>{
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
        let mut valid_names: Vec<String> = Vec::new();
        for entry in rs {
            let se = SearchEntry::construct(entry).attrs["dNSHostName"][0].clone();
            match AddressSpaceDNS::validate_dns(se.as_str()){
                Ok(()) => valid_names.push(se),
                Err(a) => return Err(a)
            };
        }
        ldap.unbind().unwrap();
        Ok(AddressSpaceDNS{ names: valid_names })
    }
    pub fn dns_from_yaml(yaml_string: &str) -> Result<AddressSpaceDNS, NameError> {
        let parsed: Vec<String> = serde_yaml::from_str(yaml_string).unwrap();
        let mut valid_names: Vec<String> = Vec::new();
        for i in parsed {
            match AddressSpaceDNS::validate_dns(i.as_str()) {
                Ok(()) => valid_names.push(i),
                Err(a) => return Err(a)
            }
        }
        return Ok(AddressSpaceDNS::new(valid_names))
    }

    pub fn dns_from_yaml_file(path: &str) -> Result<AddressSpaceDNS, NameError> {
        let yaml_string = std::fs::read_to_string(path).unwrap();
        match AddressSpaceFactory::dns_from_yaml(yaml_string.as_str()) {
            Ok(addr) => return Ok(addr),
            Err(a) => return Err(a)
        }
    }
    pub fn ipv4_range(start: Ipv4Addr, end: Ipv4Addr) -> AddressSpaceIpv4 {
        return AddressSpaceIpv4::new(start, end)
    }
    pub fn ipv4(address: Ipv4Addr) -> AddressSpaceIpv4 {
        return AddressSpaceIpv4::new(address, address)
    }
    pub fn ipv6_range(start: Ipv6Addr, end: Ipv6Addr) -> AddressSpaceIpv6 {
        return AddressSpaceIpv6::new(start, end)
    }
    pub fn ipv6(address: Ipv6Addr) -> AddressSpaceIpv6 {
        return AddressSpaceIpv6::new(address, address)
    }
}
#[cfg(test)]
pub mod as_test{
    use std::net::{Ipv4Addr, Ipv6Addr};

    use crate::probe::address_space::AddressSpaceDNS;

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
            assert!(AddressSpaceDNS::validate_dns(i).err().is_none());
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
            assert!(!(AddressSpaceDNS::validate_dns(i).err().is_none()));
        }
    }

    #[test]
    fn dns_iteration_is_ok() {
        let dns = AddressSpaceFactory::dns(vec!["SRVFUVS20311.fuvs.br".to_string(),
                                                                          "SRVFUVS22558.fuvs.br".to_string(),
                                                                          "SRVFUVS24414.fuvs.br".to_string(),
                                                                          "DIRHCSL39170.fuvs.br".to_string()]).unwrap();
        let mut last = "".to_string();
        for i in dns.names {
            assert!(i != last);
            last = i;
        }
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
        let names = AddressSpaceFactory::dns_from_ldap("SRVFUVS24414", 
                                                        None,
                                                         Some("CN=VerdeteTest,OU=SISTEMAS,OU=INFORMATICA,OU=_UNIDADE CENTRAL,DC=fuvs,DC=br"),
                                                Some("123456"),
                                                         "OU=COMPUTADORES INFORMATICA,OU=INFORMATICA,OU=_UNIDADE CENTRAL,DC=fuvs,DC=br", 
                                                     "(objectClass=computer)",
                                                       false).unwrap();
        assert!(names.names.len() > 0);
        for i in names.names {
            assert!(AddressSpaceDNS::validate_dns(i.as_str()).err().is_none());
        }
    }

    #[test]
    fn json_dns_loading_is_ok(){
        let hosts = "[\"SRVFUVS20311.fuvs.br\",\"SRVFUVS22558.fuvs.br\",\"SRVFUVS24414.fuvs.br\"]";
        let dns = AddressSpaceFactory::dns_from_json(hosts).unwrap();
        assert!(dns.names[0] == "SRVFUVS20311.fuvs.br");
        assert!(dns.names[1] == "SRVFUVS22558.fuvs.br");
        assert!(dns.names[2] == "SRVFUVS24414.fuvs.br");
        let dns2 = AddressSpaceFactory::dns_from_json_file("/etc/verdete/json_test.json").unwrap();
        assert!(dns2.names[0] == "SRVFUVS20311.fuvs.br");
        assert!(dns2.names[1] == "SRVFUVS22558.fuvs.br");
        assert!(dns2.names[2] == "SRVFUVS24414.fuvs.br");
    }

    #[test]
    fn yaml_dns_loading_is_ok(){
        let hosts = "---\n- SRVFUVS20311.fuvs.br\n- SRVFUVS22558.fuvs.br\n- SRVFUVS24414.fuvs.br";
        let dns = AddressSpaceFactory::dns_from_yaml(hosts).unwrap();
        assert!(dns.names[0] == "SRVFUVS20311.fuvs.br");
        assert!(dns.names[1] == "SRVFUVS22558.fuvs.br");
        assert!(dns.names[2] == "SRVFUVS24414.fuvs.br");
        let dns2 = AddressSpaceFactory::dns_from_yaml_file("/etc/verdete/yaml_test.yaml").unwrap();
        assert!(dns2.names[0] == "SRVFUVS20311.fuvs.br");
        assert!(dns2.names[1] == "SRVFUVS22558.fuvs.br");
        assert!(dns2.names[2] == "SRVFUVS24414.fuvs.br");
    }

    // TODO
    #[test]
    fn csv_dns_loading_is_ok(){}

    // TODO
    #[test]
    fn psql_dns_loading_is_ok(){}

    // TODO
    #[test]
    fn mysql_dns_loading_is_ok(){}

    // TODO
    #[test]
    fn mongo_dns_loading_is_ok(){}

    // TODO
    #[test]
    fn probe_dns_loading_is_ok(){}
}