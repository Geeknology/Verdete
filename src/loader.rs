use std::net::IpAddr;
use reqwest::ClientBuilder;

use crate::auth::{Auth};

#[derive(Debug, Clone)]
pub enum ResourceType <'a>{
    JSON(&'a str),
    YAML(&'a str),
    CSV(&'a str, Option<&'a str>),
    TOML(&'a str),
    XLSX
}

#[derive(Debug, Clone)]
pub struct LoaderError {}

#[derive(Debug, Clone)]
pub enum URI <'a>{
    File {path: &'a str},
    HTTP {url: &'a str, authentication: Option<Auth<'a>>, verify_certificate: bool},
    FTP {url: &'a str, authentication: Option<Auth<'a>>},
    SMB {path: &'a str, authentication: Option<Auth<'a>>},
    SCP {server: IpAddr, authentication: Option<Auth<'a>>}
}

#[derive(Debug)]
pub struct Loader {}

impl Loader {

    pub async fn load_file(path: &str) -> Result<String, LoaderError> {
        Ok(tokio::fs::read_to_string(path).await.unwrap())
    }

    pub async fn load_http(url: &str, authentication: &Option<Auth<'_>>, verify_certificate: bool) -> Result<String, LoaderError> {
        let client = ClientBuilder::new()
                                                .danger_accept_invalid_certs(!verify_certificate)
                                                .build()
                                                .unwrap();
        if let Some(auth) = authentication {
            match auth {
                Auth::Basic { username, password } => {
                    let res = client.get(url).basic_auth(username, Some(password)).send().await.unwrap();
                    return Ok(res.text().await.unwrap().to_string())
                },
                Auth::Token { token } => {
                    let res = client.get(url).bearer_auth(token).send().await.unwrap();
                    return Ok(res.text().await.unwrap().to_string())
                }
                _ => return Err(LoaderError { })
            }
            Ok("OK".to_string())
        } else {
            let res = client.get(url).send().await.unwrap();
            Ok(res.text().await.unwrap().to_string())
        }
    }

    pub async fn load_ftp(uri: &URI<'_>) -> Result<String, LoaderError> {
        Err(LoaderError {})
    }

    pub async fn load_smb(uri: &URI<'_>) -> Result<String, LoaderError> {
        Err(LoaderError {})
    }

    pub async fn load_scp(uri: &URI<'_>) -> Result<String, LoaderError> {
        Err(LoaderError {})
    }

    pub async fn load(uri: &URI<'_>) -> Result<String, LoaderError> {
        match uri {
            URI::File { path} => Loader::load_file(path).await,
            URI::HTTP { url, authentication, verify_certificate} => Loader::load_http(url, authentication, verify_certificate.to_owned()).await,
            URI::FTP { url, authentication } => Loader::load_ftp(uri).await,
            URI::SMB { path, authentication } => Loader::load_smb(uri).await,
            URI::SCP { server, authentication } => Loader::load_scp(uri).await
        }
    }
}

#[cfg(test)]
pub mod loader_test {
    use crate::auth::Auth;

    use super::{Loader, URI};

    #[tokio::test]
    async fn load_from_file(){
        let content = Loader::load(&URI::File{path: "/etc/verdete/tests/loader_file"}).await.unwrap();
        assert!(content == "Hello, world\n");
    }

    #[tokio::test]
    async fn load_from_http(){
        let content = Loader::load(&URI::HTTP { url: "http://127.0.0.1:8000/hello", authentication: None, verify_certificate: false }).await.unwrap();
        assert!(content == "Hello, world");
    }

    #[tokio::test]
    async fn load_from_http_with_basic_auth(){
        let content = Loader::load(&URI::HTTP { url: "http://127.0.0.1:8000/auth", authentication: Some(Auth::Basic { username: "test123", password: "test123" }), verify_certificate: false}).await.unwrap();
        assert!(content == "Hello, world");
    }

    #[tokio::test]
    async fn load_from_http_with_token_auth(){
        let content = Loader::load(&URI::HTTP { url: "http://127.0.0.1:8000/token", authentication: Some(Auth::Token { token: "123123" }), verify_certificate: false}).await.unwrap();
        assert!(content == "Hello, world");
    }

    #[test]
    fn load_from_http_with_spnego_auth(){
        todo!()
    }

    #[tokio::test]
    async fn load_from_http_with_x509_auth(){
        todo!()
    }

    #[test]
    fn load_from_ftp(){
        todo!()
    }

    #[test]
    fn load_from_ftp_with_basic_auth(){
        todo!()
    }

    #[test]
    fn load_from_ftp_with_spnego_auth(){
        todo!()
    }

    #[test]
    fn load_from_ftp_with_x509_auth(){
        todo!()
    }

    #[test]
    fn load_from_scp(){
        todo!()
    }

    #[test]
    fn load_from_scp_with_basic_auth(){
        todo!()
    }

    #[test]
    fn load_from_scp_with_gsssapi_auth(){
        todo!()
    }

    #[test]
    fn load_from_scp_with_x509_auth(){
        todo!()
    }

    #[test]
    fn load_from_smb(){
        todo!()
    }

    #[test]
    fn load_from_smb_with_kerberos_auth(){
        todo!()
    }

    #[test]
    fn load_from_smb_with_credssp_auth(){
        todo!()
    }
    
    #[test]
    fn load_from_smb_with_ntlm_auth(){
        todo!()
    }

    #[test]
    fn load_from_smb_with_x509_auth(){
        todo!()
    }

}