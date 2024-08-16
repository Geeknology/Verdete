use serde_json::{Map, Value};

use super::ProbeData;

pub trait Stage{
    fn execute(&self, data: &mut ProbeData);
}
#[cfg(test)]
pub mod stage_test {
    #[test]
    fn stage_execute_is_ok(){
        
    }

    #[test]
    fn stage_get_data_is_ok(){}
}