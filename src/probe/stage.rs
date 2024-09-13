use std::{borrow::{Borrow, BorrowMut}, net::IpAddr};
use serde_json::{json, Map, Value};
use crate::loader::{ResourceType, URI};
use super::{address_space::{Address, AddressSpace, AddressSpaceFactory}, ProbeData};

#[derive(Debug)]
pub enum StageNodeError {
    NodeNotFound(String),
    IndexOutOfBounds(usize)
}

#[derive(Debug)]
pub enum Stage<'a>{
    Test,
    Root,
    NodeQuery,
    AddressSpace(URI<'a>, ResourceType<'a>),
    AddressSpaceIpRange(&'a str, &'a str),
    AgentQuery
}

#[derive(Debug)]
pub struct StageNode<'a> {
    name: String,
    stage: Stage<'a>,
    next: Option<Box<StageNode<'a>>>
}

impl<'a> StageNode<'a>{
    pub fn new(name: &str, stage: Stage<'a>) -> StageNode<'a> {
        return StageNode {
            name: name.to_string(),
            stage,
            next: None
        }
    }

    pub fn next(&self) -> Option<&StageNode<'a>> {
        if let Some(node) = self.next.as_ref() {
            return Some(node.as_ref())
        } else {
            return None
        }
    }

    pub fn len(&self) -> usize {
        let mut cursor = self;
        let mut iterations = 1;
        while cursor.next().is_none() == false {
            cursor = cursor.next().unwrap();
            iterations += 1;
        }
        return iterations as usize
    }

    pub fn insert_node_as_next_of(&mut self, name: &str, node: StageNode<'a>) -> Result<(), StageNodeError> {
        if self.name == name {
            self.next = Some(Box::new(node));
            return Ok(())
        }
        let mut cursor = self;
        while cursor.next().is_none() == false {
            if cursor.next().unwrap().name == name {
                cursor.next.as_mut().unwrap().next = Some(Box::new(node));
                return Ok(())
            }
            cursor = cursor.next.as_mut().unwrap().as_mut();
        }
        return Err(StageNodeError::NodeNotFound(name.to_string()))
    }

    pub fn insert_node_at_index(&mut self, index: usize, node: StageNode<'a>) -> Result<(), StageNodeError> {
        if index == 0 {
            self.next = Some(Box::new(node));
            return Ok(())
        }
        let mut len = self.len();
        if index > len - 1 {
            return Err(StageNodeError::IndexOutOfBounds(index))
        }
        let mut cursor = self;
        let mut iterations = 0;
        while iterations < index {
            iterations += 1;
            cursor = cursor.next.as_mut().unwrap().as_mut();
        }
        cursor.next = Some(Box::new(node));
        return Ok(())
    }

    pub fn get_node(&self, name: &str) -> Option<&StageNode<'a>> {
        if self.name == name {
            return Some(self)
        }
        let mut cursor = self;
        while cursor.next().is_none() == false {
            if cursor.next().unwrap().name == name {
                return Some(cursor.next().unwrap())
            }
            cursor = cursor.next().unwrap()
        }
        return None
    }

    pub fn append_node(&mut self, node: StageNode<'a>) {
        let mut cursor = self;
        while cursor.next().is_none() == false {
            cursor = cursor.next.as_mut().unwrap().as_mut();
        }
        cursor.next = Some(Box::new(node));
    }

    pub fn del_node(&mut self, name: &str) -> Result<(), StageNodeError> {
        if self.name == name {
            return Err(StageNodeError::NodeNotFound(name.to_string()))
        }
        let mut cursor = self;
        while cursor.next().is_none() == false {
            if cursor.next().unwrap().name == name {
                cursor.next = None;
                return Ok(())
            }
            cursor = cursor.next.as_mut().unwrap().as_mut()
        }
        return Err(StageNodeError::NodeNotFound(name.to_string()))
    }

    pub async fn execute(&self, probe_data: &mut ProbeData){
        println!("Stage {:?} being executed", self.name);
        match &self.stage {
            Stage::AddressSpace(a, b) => {
                probe_data.address_space["type"] = json!("address_list");
                probe_data.address_space["addressess"] = json!(AddressSpaceFactory::from(a.clone(), b.clone()).await.unwrap());
            },
            Stage::AddressSpaceIpRange(a, b) => {
                probe_data.address_space.insert("type".to_string(), json!("ip_range".to_string()));
                probe_data.address_space.insert("addresses".to_string(), json!(AddressSpaceFactory::ip_range(Address::from_str(a).unwrap(), Address::from_str(b).unwrap())));
            },
            Stage::Root => {
                println!("I'm the root node");
            },
            _ => panic!("Not Implemented Yet")
        }
    }
}

#[cfg(test)]
pub mod stage_test {
    use super::{Stage, StageNode};


    #[test]
    fn stage_list_iteration_is_ok(){}

    #[test]
    fn stage_list_add_node_is_ok(){
        let mut root = StageNode::new("Test", Stage::Test);
        let node_01 = StageNode::new("Test01", Stage::Test);
        root.append_node(node_01);
        assert!(root.next().unwrap().name == "Test01");
        let node_02 = StageNode::new("Test02", Stage::NodeQuery);
        root.append_node(node_02);
        assert!(root.next().unwrap().next().unwrap().name == "Test02");
    }

    
    #[test]
    fn stage_list_get_node_by_name_is_ok(){
        let mut root = StageNode::new("Test", Stage::Test);
        let node_01 = StageNode::new("Test01", Stage::Test);
        root.append_node(node_01);
        assert!(root.get_node("Test01").is_none() == false);
        let node_02 = StageNode::new("Test02", Stage::NodeQuery);
        root.append_node(node_02);
        assert!(root.get_node("Test02").is_none() == false);
        assert!(root.get_node("Test03").is_none() == true);
    }


    #[test]
    fn stage_list_del_node_is_ok(){
        let mut root = StageNode::new("Test", Stage::Test);
        let node_01 = StageNode::new("Test01", Stage::Test);
        let node_02 = StageNode::new("Test02", Stage::NodeQuery);
        root.append_node(node_01);
        root.append_node(node_02);
        assert!(root.get_node("Test01").is_none() == false);
        assert!(root.get_node("Test02").is_none() == false);
        root.del_node("Test02");
        assert!(root.get_node("Test02").is_none() == true);
        let node_03 = StageNode::new("Test03", Stage::NodeQuery);
        root.append_node(node_03);
        root.del_node("Test01");
        assert!(root.get_node("Test01").is_none() == true);
        assert!(root.get_node("Test03").is_none() == true);
    }

    #[test]
    fn stage_insert_node_at_index() {
        let mut root = StageNode::new("Test", Stage::Test);
        let node_01 = StageNode::new("Test01", Stage::Test);
        let node_02 = StageNode::new("Test02", Stage::NodeQuery);
        root.insert_node_at_index(0, node_01);
        assert!(root.next().unwrap().name == "Test01");
        root.insert_node_at_index(1, node_02);
        assert!(root.next().unwrap().next().unwrap().name == "Test02");
        let node_03 = StageNode::new("Test03", Stage::NodeQuery);
        assert!(root.insert_node_at_index(3, node_03).is_ok() == false);
    }

    #[test]
    fn stage_insert_node_after() {
        let mut root = StageNode::new("Test", Stage::Test);
        let node_01 = StageNode::new("Test01", Stage::Test);
        let node_02 = StageNode::new("Test02", Stage::NodeQuery);
        root.insert_node_as_next_of("Test", node_01);
        assert!(root.next().unwrap().name == "Test01");
        root.insert_node_as_next_of("Test01", node_02);
        assert!(root.next().unwrap().next().unwrap().name == "Test02");
        let node_03 = StageNode::new("Test03", Stage::NodeQuery);
        assert!(root.insert_node_as_next_of("Test10", node_03).is_ok() == false);
    }

    #[test]
    fn stage_loading_is_ok(){}

    #[test]
    fn stage_module_loading_is_ok(){}
}