mod module;
mod auto_reg;
mod address_space;
mod conditional;
mod net_test;
mod node_query;
mod passive;
mod stage;
mod store;
mod preprocess;

use std::{any::Any, collections::HashMap};

use address_space::AddressSpace;
use serde_json::{Map, Value};
use stage::Stage;

#[derive(Debug)]
pub struct ProbeData {
    stages: Map<String, Value>,
    results: Map<String, Value>
}

impl ProbeData {
    fn new() -> ProbeData{
        return ProbeData {
            stages: Map::new(),
            results: Map::new()
        }
    }
}

pub struct Probe<T> where T: AddressSpace {
    address_space: T,
    stages: Vec<Box<dyn Stage>>,
    data: ProbeData
}

impl<T> Probe<T> where T: AddressSpace {
    fn new(address_space: T) -> Probe<T>{
        return Probe {
            address_space,
            stages: Vec::new(),
            data: ProbeData::new()
        }
    }
    
    fn add_stage(&mut self, stage: Box<dyn Stage>, index: usize) {
        self.stages.insert(index, stage);
    }

    fn execute(&mut self) {
        for i in self.stages.iter() {
            i.execute(&mut self.data);
        }
    }

    fn get_stage_data(&self, stage_name: &str) -> Option<&Value> {
        return self.data.stages.get(stage_name)
    }

    fn get_results_data(&self, stage_name: &str) -> Option<&Value> {
        return self.data.results.get(stage_name)
    }
}
#[cfg(test)]
pub mod probe_test {
    use std::any::{type_name_of_val, Any};
    use std::borrow::{Borrow, BorrowMut};
    use std::collections::HashMap;

    use serde_json::{json, Map, Number, Value};

    use crate::probe::ProbeData;

    use super::address_space::AddressSpaceFactory;
    use super::stage::Stage;
    use super::Probe;

    struct TestStage {}

    impl Stage for TestStage{
        fn execute(&self, data: &mut ProbeData) {
            let mut result: Map<String, Value> = Map::new();
            result.insert("Is_This_a_Test".to_string(), Value::Bool(true));
            result.insert("This_is_Five".to_string(), json!(5));
            result.insert("This_is_a_String".to_string(), json!("Hello, Friend"));
            result.insert("This_is_an_Array".to_string(), Value::Array(vec![json!(1), json!(2), json!(3)]));
            result.insert("This_is_Another_Object".to_string(), json!(Map::new()));
            data.stages.insert("TestStage".to_string(), Value::Number(Number::from(0)));
            data.results.insert("TestStage".to_string(), json!(result));
        }
    }

    /*
    #[test]
    fn probe_construction_ok(){
        let addr_space = AddressSpaceFactory::dns(vec!["SRVFUVS24414.fuvs.br".to_string(), "SRVFUVS22558.fuvs.br".to_string(), "SRVFUVS20311.fuvs.br".to_string()]).unwrap();
        let mut probe = Probe::new(addr_space);
        assert!(probe.address_space.names.contains(&"SRVFUVS24414.fuvs.br".to_string()));
        assert!(probe.address_space.names.len() == 3);
        assert!(probe.stages.len() == 0);
        let stage = TestStage {};
        probe.add_stage(Box::new(stage), 0);
        assert!(probe.stages.len() == 1);
    }
    */
/*
    #[test]
    fn probe_add_data_ok() {
        let test_stage = TestStage {};
        let addr_space = AddressSpaceFactory::(vec!["SRVFUVS24414.fuvs.br".to_string(), "SRVFUVS22558.fuvs.br".to_string(), "SRVFUVS20311.fuvs.br".to_string()]).unwrap();
        let mut probe = Probe::new(addr_space);
        probe.add_stage(Box::new(test_stage), 0);
        probe.execute();
        assert!(probe.get_stage_data("TestStage").unwrap() == 0);
        assert!(probe.get_results_data("TestStage").unwrap()["Is_This_a_Test"] == true);
        assert!(probe.get_results_data("TestStage").unwrap()["This_is_Five"] == 5);
        assert!(probe.get_results_data("TestStage").unwrap()["This_is_a_String"] == "Hello, Friend");
        assert!(probe.get_results_data("TestStage").unwrap()["This_is_an_Array"] == json!(vec![1, 2, 3]));
        assert!(probe.get_results_data("TestStage").unwrap()["This_is_Another_Object"] == json!(Map::new()))
    }
*/
    #[test]
    fn no_module_linear_pipeline_ok(){}

    #[test]
    fn linear_pipeline_with_module_ok(){}

    #[test]
    fn pipeline_stage_data_integrity_is_ok(){}

    #[test]
    fn pipeline_stage_data_is_right_order(){}

    #[test]
    fn no_module_branching_pipeline_is_ok(){}

    #[test]
    fn branching_pipeline_with_module_is_ok(){}

    #[test]
    fn preprocess_pipeline_data_integrity_is_ok(){}

    #[test]
    fn preprocess_pipeline_with_branching_data_integrity_is_ok(){}

    #[test]
    fn recursive_pipeline_is_ok(){}

    #[test]
    fn recursive_pipeline_data_is_ok(){}

    #[test]
    fn recursive_pipeline_data_integrity_is_ok(){}
}