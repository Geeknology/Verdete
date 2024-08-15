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

pub mod probe{}

#[cfg(test)]
pub mod probe_test {
    #[test]
    fn probe_construction_ok(){}

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