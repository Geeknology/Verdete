mod module;
mod auto_reg;
mod address_space;
mod filtering;
mod net_test;
mod node_query;
mod passive;
mod stage;
mod store;
mod preprocess;

use std::{any::Any, borrow::Borrow, collections::HashMap, ops::Add};

use address_space::AddressSpace;
use serde_json::{json, Map, Value};
use stage::Stage;

#[derive(Debug)]
pub struct ProbeDataError {}

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

    fn set_stage_data(&mut self, stage_name: &str, data: Map<String, Value>) -> Result<(), ProbeDataError>{
        self.stages.insert(stage_name.to_string(), Value::Object(data));
        Ok(())
    }
}

pub struct Probe <T> where T: AddressSpace{
    address_space: T,
    stages: Vec<Stage>,
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
    
    /*fn add_stage(&mut self, stage: Box<dyn Stage>, index: usize) {
        self.stages.insert(index, stage);
    }

    fn execute(&mut self) {
        for i in self.stages.iter() {
            for j in self.address_space.iter() {
                let data = i.execute(&j, &mut self.data.results);
                self.data.set_stage_data(i.get_name(), data).unwrap();
            }
        }
    }*/

    fn get_stage_data(&self, stage_name: &str) -> Option<&Value> {
        return self.data.stages.get(stage_name)
    }

    fn get_results_data(&self, stage_name: &str) -> Option<&Value> {
        return self.data.results.get(stage_name)
    }

    fn stage_data(execution_time_total_s: f64, execution_time_avg: f64, execution_status: bool, execution_status_code: u16) -> Map<String, Value>{
        let mut data = Map::new();
        data.insert("execution_time_total_s".to_string(), json!(execution_time_total_s));
        data.insert("execution_time_avg".to_string(), json!(execution_time_avg));
        data.insert("execution_status".to_string(), json!(execution_status));
        data.insert("execution_status_code".to_string(), json!(execution_status_code));
        return data
    }
}
#[cfg(test)]
pub mod probe_test {
    use serde_json::{json, Map, Number, Value};
    use crate::loader::URI;
    use crate::probe::ProbeData;
    use super::address_space::{Address, AddressSpace, AddressSpaceFactory};
    use super::stage::Stage;
    use super::Probe;
/*
    #[tokio::test]
    async fn probe_construction_ok(){
        let addr_space = AddressSpaceFactory::from(URI::File { path: "/etc/verdete/json_dns_list.json" }, crate::loader::ResourceType::JSON("def.hosts")).await.unwrap();
        let mut probe = Probe::new(Box::new(addr_space));
        assert!(probe.stages.len() == 0);
        assert!(probe.stages.len() == 1);
    }
    #[tokio::test]
    async fn probe_add_data_ok() {
        let test_stage = TestStage {
            stage_name: "TestStage".to_string()
        };
        let addr_space = AddressSpaceFactory::from(URI::File { path: "/etc/verdete/json_dns_list.json" }, crate::loader::ResourceType::JSON("def.hosts")).await.unwrap();
        let mut probe = Probe::new(Box::new(addr_space));
        probe.add_stage(Box::new(test_stage), 0);
        let test_stage2 = TestStage {
            stage_name: "TestStage2".to_string()
        };
        probe.add_stage(Box::new(test_stage2), 1);
        probe.execute();
        println!("{:?}", probe.data);
        assert!(probe.get_stage_data("TestStage").unwrap().get("execution_time_total_s").unwrap().as_f64().unwrap() == 10.0);
        assert!(probe.get_results_data("TestStage").unwrap()["Is_This_a_Test"] == true);
        assert!(probe.get_results_data("TestStage").unwrap()["This_is_Five"] == 5);
        assert!(probe.get_results_data("TestStage").unwrap()["This_is_a_String"] == "Hello, Friend");
        assert!(probe.get_results_data("TestStage").unwrap()["This_is_an_Array"] == json!(vec![1, 2, 3]));
        assert!(probe.get_results_data("TestStage").unwrap()["This_is_Another_Object"] == json!(Map::new()));
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