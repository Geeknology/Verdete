use std::{net::{Ipv4Addr, Ipv6Addr}, str::FromStr, u8};

use ldap3::{LdapConn, SearchEntry};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::loader::{Loader, ResourceType, URI};

#[derive(Debug)]
pub struct AddrError{}

#[derive(Debug)]
pub struct AddressSpaceError{}

pub trait AddressSpace{
    fn iter(&mut self) -> impl Iterator<Item = Address>;
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressSpaceIpRange{
    start: Address,
    end: Address,
    curr: Address,
    next: Address
}

impl AddressSpaceIpRange{
    fn new(start: Address, end: Address) -> Result<AddressSpaceIpRange, AddressSpaceError> {
        if std::mem::discriminant(&start) == std::mem::discriminant(&end) {
            Ok(AddressSpaceIpRange {
                start: start.clone(),
                end: end.clone(),
                curr: start.clone(),
                next: start.clone()
            })
        } else {
            Err(AddressSpaceError {  })
        }
    }
}

impl<'a> Iterator for AddressSpaceIpRange{
    type Item = Address;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = self.next.clone();
        if self.curr.is_ipv4() {
            if self.curr.ipv4().unwrap() > self.end.ipv4().unwrap() {
                return None
            }
            self.next = Address::Ipv4(Ipv4Addr::from_bits(self.curr.ipv4().unwrap().to_bits() + 1));
            Some(self.curr.clone())
        } else {
            if self.curr.ipv6().unwrap() > self.end.ipv6().unwrap() {
                return None
            }
            self.next = Address::Ipv6(Ipv6Addr::from_bits(self.curr.ipv6().unwrap().to_bits() + 1));
            Some(self.curr.clone())
        }
    }
}

impl AddressSpace for AddressSpaceIpRange{
    fn iter(&mut self) -> impl Iterator<Item = Address> {
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressSpaceAddrList {
    pub addrs: Vec<Address>,
    curr: usize,
    next: usize
}

impl AddressSpaceAddrList {
    fn new(addrs: Vec<Address>) -> AddressSpaceAddrList {
        AddressSpaceAddrList {
            addrs,
            curr: 0,
            next: 0
        }
    }

    pub fn contains(&self, addr: &Address) -> bool {
        self.addrs.contains(addr)
    }

    pub fn len(&self) -> usize {
        self.addrs.len()
    }
}

impl Iterator for AddressSpaceAddrList {
    type Item = Address;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = self.next;
        if self.next >= self.len() {
            return None
        }
        self.next += 1;
        return Some(self.addrs.get(self.curr).unwrap().to_owned())
    }
}

impl AddressSpace for AddressSpaceAddrList{
    fn iter(&mut self) -> impl Iterator<Item = Address> {
        self
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Address {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
    DNS(String)
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
    
    fn ipv4_from_str(s: &str) -> Result<Address, AddrError> {
        match Ipv4Addr::from_str(s) {
            Ok(addr) => Ok(Address::Ipv4(addr)),
            Err(err) => Err(AddrError { })
        }
    }

    fn ipv6_from_str(s: &str) -> Result<Address, AddrError> {
        match Ipv6Addr::from_str(s) {
            Ok(addr) => Ok(Address::Ipv6(addr)),
            Err(err) => Err(AddrError { })
        }
    }

    fn dns_from_str(s: &str) -> Result<Address, AddrError> {
        match Address::validate_dns(s) {
            Ok(()) => Ok(Address::DNS(s.to_string())),
            Err(err) => Err(err)
        }
    }

    pub fn from_str(s: &str) -> Result<Address, AddrError> {
        match Address::ipv6_from_str(s) {
            Ok(ipv6) => Ok(ipv6),
            Err(_) => match Address::ipv4_from_str(s) {
                                    Ok(addr) => Ok(addr),
                                    Err(_) => match Address::dns_from_str(s) {
                                        Ok(addr) => Ok(addr),
                                        Err(e) => Err(e)
                                    }
            }
        }
    }

    pub fn ipv4(&self) -> Result<&Ipv4Addr, AddrError>{
        match &self {
            Address::Ipv4(a) => Ok(a),
            _ => Err(AddrError {})
        }
    }

    pub fn ipv6(&self) -> Result<&Ipv6Addr, AddrError> {
        match &self {
            Address::Ipv6(a) => Ok(a),
            _ => Err(AddrError {})
        }
    }

    pub fn dns(&self) -> Result<&String, AddrError> {
        match &self {
            Address::DNS(a) => Ok(a),
            _ => Err(AddrError {})
        }
    }

    pub fn is_ipv4(&self) -> bool {
        match &self {
            Address::Ipv4(_) => true,
            _ => false
        }
    }

    pub fn is_ipv6(&self) -> bool {
        match &self {
            Address::Ipv6(_) => true,
            _ => false
        }
    }

    pub fn is_dns(&self) -> bool {
        match &self {
            Address::DNS(_) => true,
            _ => false
        }
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        match &self {
            Address::DNS(a) => a.to_string(),
            Address::Ipv4(a) => a.to_string(),
            Address::Ipv6(a) => a.to_string()
        }
    }
}

#[derive(Debug)]
pub struct AddressSpaceFactory{}

impl AddressSpaceFactory{
    pub async fn from<'a>(uri: URI<'a>, resource_type: ResourceType<'a>) -> Result<AddressSpaceAddrList, AddressSpaceError> {
        let content = Loader::load(&uri).await.unwrap();
        match resource_type {
            ResourceType::CSV(column, sep) => AddressSpaceFactory::from_csv(content.as_str(), column, sep),
            ResourceType::JSON(selector) => AddressSpaceFactory::from_json(content.as_str(), selector),
            ResourceType::YAML(selector) => AddressSpaceFactory::from_yaml(content.as_str(), selector),
            ResourceType::TOML(selector) => AddressSpaceFactory::from_toml(content.as_str(), selector),
            //ResourceType::CSV(column, sep) => AddressSpaceFactory::from_csv(content.as_str(), column, sep),
            _ => Err(AddressSpaceError {})
        }
    }

    pub fn from_json(uri: &str, selector: &str) -> Result<AddressSpaceAddrList, AddressSpaceError>{
        if selector.is_empty() {
            return Err(AddressSpaceError {  })
        }
        let json: Value = serde_json::from_str(uri).unwrap();
        let parsed_selector: Vec<&str> = selector.split(".").collect();
        let mut cursor = &json;
        for i in parsed_selector {
            cursor = cursor.get(i).unwrap();
        }
        let addrs_seq = cursor.as_array().unwrap();
        if addrs_seq.len() <= 0 {
            Err(AddressSpaceError { })
        } else {
            let mut addrs: Vec<Address> = Vec::new();
            for i in addrs_seq {
                addrs.push(Address::from_str(i.as_str().unwrap()).unwrap())
            }
            Ok(AddressSpaceAddrList::new(addrs))
        }
    }
    
    pub fn from_yaml(yaml_string: &str, selector: &str) -> Result<AddressSpaceAddrList, AddressSpaceError> {
        if selector.is_empty() {
            return Err(AddressSpaceError {  })
        }
        let yaml: serde_yaml::Value = serde_yaml::from_str(yaml_string).unwrap();
        let parsed_selector: Vec<&str> = selector.split(".").collect();
        let mut cursor = &yaml;
        for i in parsed_selector {
            cursor = cursor.get(i).unwrap();
        }
        let addrs_seq = cursor.as_sequence().unwrap();
        if addrs_seq.is_empty() {
            Err(AddressSpaceError {  })
        }else {
            let mut addrs: Vec<Address> = Vec::new();
            for i in addrs_seq {
                addrs.push(Address::from_str(i.as_str().unwrap()).unwrap())
            }
            Ok(AddressSpaceAddrList::new(addrs))
        }
    }
    
    pub fn from_toml(toml_string: &str, selector: &str) -> Result<AddressSpaceAddrList, AddressSpaceError> {
        if selector.is_empty() {
            return Err(AddressSpaceError { })
        }
        let toml: toml::Value = toml::from_str(toml_string).unwrap();
        let parsed_selector: Vec<&str> = selector.split(".").collect();
        let mut cursor = &toml;
        for i in parsed_selector {
            cursor = cursor.get(i).unwrap();
        }
        let addrs_seq = cursor.as_array().unwrap();
        if addrs_seq.is_empty() {
            Err(AddressSpaceError {  })
        } else {
            let mut addrs: Vec<Address> = Vec::new();
            for i in addrs_seq {
                addrs.push(Address::from_str(i.as_str().unwrap()).unwrap())
            }
            Ok(AddressSpaceAddrList::new(addrs))
        }
    }
    
    pub fn from_csv(csv_string: &str, column: &str, sep: Option<&str>) -> Result<AddressSpaceAddrList, AddressSpaceError> {
        if column.is_empty() {
            return Err(AddressSpaceError {})
        }
        let mut csv = csv::ReaderBuilder::new().delimiter(sep.unwrap_or(",").as_bytes()[0]).from_reader(csv_string.as_bytes());
        let headers = csv.headers().unwrap();
        let index = headers.as_slice().find(column).unwrap();
        let mut addrs: Vec<Address> = Vec::new();
        while let Some(results) = csv.records().next() {
            addrs.push(Address::from_str(&results.unwrap()[index]).unwrap());
        }
        Ok(AddressSpaceAddrList::new(addrs))
    }
    /*

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
            base_dn,
            ldap3::Scope::Subtree,
            ldap_filter,
            ["DnsHostName"]
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
    pub fn ip_range(start: Address, end: Address) -> AddressSpaceIpRange {
        AddressSpaceIpRange::new(start, end).unwrap()
    }
}
#[cfg(test)]
pub mod as_test{
    use std::net::Ipv6Addr;

    use crate::{loader::{ResourceType, URI}, probe::address_space::Address};

    use super::AddressSpaceFactory;
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
            assert!(Address::validate_dns(i).err().is_some());
        }
    }

    #[tokio::test]
    async fn ipv4_list_from_json_is_ok() {
        let ipv4_list = AddressSpaceFactory::from(
                                                                    URI::File { path: "/etc/verdete/json_ipv4_list.json" }, 
                                                                    ResourceType::JSON("def.hosts")
                                                                ).await.unwrap();
        assert!(ipv4_list.len() == 10);
        for i in ipv4_list {
            assert!(i.is_ipv4());
        }
    }

    #[tokio::test]
    async fn ipv6_list_from_json_is_ok() {
        let ipv6_list = AddressSpaceFactory::from(
                                                                    URI::File { path: "/etc/verdete/json_ipv6_list.json" }, 
                                                                    ResourceType::JSON("def.hosts")
                                                                ).await.unwrap();
        assert!(ipv6_list.len() == 10);
        for i in ipv6_list {
            assert!(i.is_ipv6());
        }
    }

    #[tokio::test]
    async fn dns_list_from_json_is_ok() {
        let dns_list = AddressSpaceFactory::from(
                                                                    URI::File { path: "/etc/verdete/json_dns_list.json" }, 
                                                                    ResourceType::JSON("def.hosts")
                                                                ).await.unwrap();
        assert!(dns_list.len() == 5);
        for i in dns_list {
            assert!(i.is_dns());
        }
    }

    #[tokio::test]
    async fn ipv4_list_from_yaml_is_ok() {
        let ipv4_list = AddressSpaceFactory::from(
            URI::File { path: "/etc/verdete/yaml_ipv4_list.yaml" },
            ResourceType::YAML("def.hosts")
        ).await.unwrap();
        assert!(ipv4_list.len() == 10);
        for i in ipv4_list {
            assert!(i.is_ipv4());
        }
    }

    #[tokio::test]
    async fn ipv6_list_from_yaml_is_ok() {
        let ipv6_list = AddressSpaceFactory::from(
            URI::File { path: "/etc/verdete/yaml_ipv6_list.yaml" },
            ResourceType::YAML("def.hosts")
        ).await.unwrap();
        assert!(ipv6_list.len() == 5);
        for i in ipv6_list {
            assert!(i.is_ipv6());
        }
    }

    #[tokio::test]
    async fn dns_list_from_yaml_is_ok() {
        let dns_list = AddressSpaceFactory::from(
            URI::File { path: "/etc/verdete/yaml_dns_list.yaml"},
            ResourceType::YAML("def.hosts")
        ).await.unwrap();
        assert!(dns_list.len() == 5);
        for i in dns_list {
            assert!(i.is_dns());
        }
    }

    #[tokio::test]
    async fn ipv4_list_from_toml_is_ok() {
        let ipv4_list = AddressSpaceFactory::from(
            URI::File { path: "/etc/verdete/toml_ipv4_list.toml" },
            ResourceType::TOML("def.hosts")
        ).await.unwrap();
        assert!(ipv4_list.len() == 10);
        for i in ipv4_list {
            assert!(i.is_ipv4())
        }
    }

    #[tokio::test]
    async fn ipv6_list_from_toml_is_ok() {
        let ipv6_list = AddressSpaceFactory::from(
            URI::File { path: "/etc/verdete/toml_ipv6_list.toml" },
            ResourceType::TOML("def.hosts")
        ).await.unwrap();
        assert!(ipv6_list.len() == 5);
        for i in ipv6_list {
            assert!(i.is_ipv6())
        }
    }

    #[tokio::test]
    async fn dns_list_from_toml_is_ok() {
        let dns_list = AddressSpaceFactory::from(
            URI::File { path: "/etc/verdete/toml_dns_list.toml" },
            ResourceType::TOML("def.hosts")
        ).await.unwrap();
        assert!(dns_list.len() == 5);
        for i in dns_list {
            assert!(i.is_dns());
        }
    }

    #[tokio::test]
    async fn ipv4_list_from_csv_is_ok() {
        let ipv4_list = AddressSpaceFactory::from(
            URI::File { path: "/etc/verdete/csv_ipv4_list.csv"},
            ResourceType::CSV("hosts", None)
        ).await.unwrap();
        assert!(ipv4_list.len() == 10);
        for i in ipv4_list {
            assert!(i.is_ipv4());
        }
    }

    #[tokio::test]
    async fn ipv6_list_from_csv_is_ok() {
        let ipv6_list = AddressSpaceFactory::from(
            URI::File { path: "/etc/verdete/csv_ipv6_list.csv" },
            ResourceType::CSV("hosts", None)
        ).await.unwrap();
        assert!(ipv6_list.len() == 3);
        for i in ipv6_list {
            assert!(i.is_ipv6())
        }
    }

    #[tokio::test]
    async fn dns_list_from_csv_is_ok() {
        let dns_list = AddressSpaceFactory::from(
            URI::File { path: "/etc/verdete/csv_dns_list.csv" },
            ResourceType::CSV("hosts", None)
        ).await.unwrap();
        assert!(dns_list.len() == 5);
        for i in dns_list {
            assert!(i.is_dns());
        }
    }

    #[test]
    fn ipv4_range_iteration_is_ok(){
        let range = AddressSpaceFactory::ip_range(Address::ipv4_from_str("192.168.0.1").unwrap(), Address::ipv4_from_str("192.168.0.255").unwrap());
        let mut fourth_octet = 0;
        for i in range {
            assert!(i.ipv4().unwrap().octets()[3] > fourth_octet);
            fourth_octet = i.ipv4().unwrap().octets()[3];
        }

        let range = AddressSpaceFactory::ip_range(Address::ipv4_from_str("192.168.0.1").unwrap(), Address::ipv4_from_str("192.168.10.0").unwrap());
        let mut third_octet = 0;
        let mut fourth_octet = 0;
        for i in range {
            if i.ipv4().unwrap().octets()[3] < fourth_octet {
                assert!(i.ipv4().unwrap().octets()[2] > third_octet)
            } else {
                assert!(i.ipv4().unwrap().octets()[3] > fourth_octet);
            }
            third_octet = i.ipv4().unwrap().octets()[2];
            fourth_octet = i.ipv4().unwrap().octets()[3];
        }
    }

    #[test]
    fn ipv6_range_iteration_is_ok(){
        let range = AddressSpaceFactory::ip_range(Address::ipv6_from_str("::1").unwrap(), Address::ipv6_from_str("::FFFF").unwrap());
        let mut last = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
        for i in range {
            assert!(*i.ipv6().unwrap() > last);
            last = *i.ipv6().unwrap();
        }

        let range = AddressSpaceFactory::ip_range(Address::ipv6_from_str("::1").unwrap(), Address::ipv6_from_str("::1:FFFF").unwrap());
        let mut last = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
        for i in range {
            assert!(*i.ipv6().unwrap() > last);
            last = *i.ipv6().unwrap();
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
        assert!(names.len() > 0);
    }
}