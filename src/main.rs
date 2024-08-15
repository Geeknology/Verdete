pub mod probe;
pub mod ansible;
pub mod config;
pub mod logging;
pub mod database;
pub mod web;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
pub mod main_test {
    #[test]
    fn http_loading_is_ok(){}

    #[test]
    fn logging_backend_loading_is_ok(){}

    #[test]
    fn database_connection_is_ok(){}

    #[test]
    fn probes_loading_is_ok(){}

    #[test]
    fn config_file_loading_is_ok(){}

    #[test]
    fn multiprocessing_is_ok(){}
}