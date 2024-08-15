pub trait Stage {
    async fn execute(){}
    async fn get_data(){}
}
#[cfg(test)]
pub mod stage_test {
    #[test]
    fn stage_execute_is_ok(){
        
    }

    #[test]
    fn stage_get_data_is_ok(){}
}