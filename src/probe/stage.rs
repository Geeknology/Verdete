use std::borrow::{Borrow, BorrowMut};

use serde_json::{Map, Value};

use crate::loader::{ResourceType, URI};

use super::address_space::{Address, AddressSpace};

#[derive(Debug)]
pub enum StageTreeError{
    NodeNotFound(String)
}

#[derive(Debug)]
pub struct StageTree<'a> {
    root: StageNode<'a>,
    len: usize
}

impl<'a> StageTree<'a> {
    pub fn new() -> StageTree<'a> {
        return StageTree {
            root: StageNode::new("root", Stage::Root),
            len: 0
        }
    }

    pub fn get_node(&self, name: String) -> Result<&'a StageNode<'a>, StageTreeError> {
        let len = self.len();
        let mut cursor = &mut self.root;
        let mut iterations = 0;
        while cursor.next().is_none() == false{
            if iterations >= len {
                return Err(StageTreeError::NodeNotFound(name))
            }
            if cursor.next().unwrap().name == name {
                return Ok(cursor.next().unwrap())
            }
            cursor = cursor.next().unwrap();
            iterations += 1;
        }
        return Err(StageTreeError::NodeNotFound(name))
    }

    pub fn append_node(&mut self, node: StageNode<'a>){
        let len = self.len();
        let mut cursor = &mut self.root;
        for _ in 0..len {
            cursor = cursor.next().unwrap();
        }
        cursor.set_next(node);
        self.len += 1;
    }

    pub fn del_node(&mut self, name: String) -> Result<(), StageTreeError>{
        let len = self.len();
        let mut cursor = &mut self.root;
        let mut iterations = 0;
        while cursor.next().is_none() == false{
            if iterations >= len {
                return Err(StageTreeError::NodeNotFound(name))
            }
            cursor = cursor.next().unwrap();
            iterations += 1;
            if cursor.next().unwrap().name == name {
                cursor.del_next();
                self.len -= 1;
                return Ok(())
            }
        }
        return Err(StageTreeError::NodeNotFound(name))
    }

    pub fn len(&self) -> usize {
        return self.len
    }
}

#[derive(Debug)]
pub struct StageNode<'a> {
    name: String,
    stage: Stage<'a>,
    next: Option<Box<StageNode<'a>>>
}

impl<'a> StageNode<'a> {
    pub fn new(name: &str, stage: Stage<'a>) -> StageNode<'a> {
        return StageNode {
            name: name.to_string(),
            stage,
            next: None
        }
    }

    pub fn next(&mut self) -> Option<&mut StageNode<'a>> {
        if let Some(next) = self.next.as_mut() {
            return Some(self.next.as_mut().unwrap().as_mut())
        }else {
            return None
        }
    }

    pub fn set_next(&mut self, node: StageNode<'a>) {
        self.next = Some(Box::new(node));
    }

    pub fn del_next(&mut self) {
        self.next = None;
    }
}

#[derive(Debug)]
pub enum Stage<'a>{
    AddressSpace(URI<'a>, ResourceType<'a>),
    Root,
    NodeQuery,
    AgentQuery,
    Passive,
    AutoReg,
    ExternalQuery,
    Filter,
    Reduce,
    Conditional,
    Preprocess,
    Store,
}

#[cfg(test)]
pub mod stage_test {
    use super::{Stage, StageNode, StageTree};

    #[test]
    fn stage_list_iteration_is_ok(){}

    #[test]
    fn stage_list_add_node_is_ok(){
        let mut tree = StageTree::new();
        let mut node_01 = StageNode::new("Test", Stage::AgentQuery);
        tree.append_node(node_01);
        assert!(tree.root.next().unwrap().name == "Test");
        let mut node_02 = StageNode::new("Test02", Stage::AgentQuery);
        tree.append_node(node_02);
        assert!(tree.root.next().unwrap().next().unwrap().name == "Test02");}

    #[test]
    fn stage_list_del_node_is_ok(){}

    #[test]
    fn stage_list_get_node_by_name_is_ok(){
        let mut tree = StageTree::new();
        let mut node_01 = StageNode::new("Test01", Stage::AgentQuery);
        let mut node_02 = StageNode::new("Test02", Stage::AgentQuery);
        let mut node_03 = StageNode::new("Test03", Stage::AgentQuery);
        tree.append_node(node_01);
        tree.append_node(node_02);
        tree.append_node(node_03);
        assert!(tree.get_node("Test01".to_string()).next().unwrap().name == "Test02");
        assert!(tree.get_node("Test02".to_string()).next().unrwap().name == "Test03");
        assert!(tree.get_node("Test03".to_string()).next().is_none() == true);
        assert!(tree.get_node("Test04".to_string()).is_ok() == false);
    }

    #[test]
    fn stage_loading_is_ok(){}

    #[test]
    fn stage_module_loading_is_ok(){}
}