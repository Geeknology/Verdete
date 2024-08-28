#[derive(Debug)]
pub enum Auth<'a> {
    Basic {username: &'a str, password: &'a str},
    Kerberos {username: &'a str, password: &'a str},
    X509 {certificate_path: &'a str, private_key: &'a str},
    NTLM {username: &'a str, password: &'a str},
    Token {token: &'a str}
}

impl<'a> Auth<'a> {
    pub fn authenticate(&self, uri: &'a str) {

    }
}

#[cfg(test)]
pub mod auth_test {}