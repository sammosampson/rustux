
use crate::prelude::*;


pub struct AbstractSyntaxGraph {
    nodes: Vec<AbstractSyntaxGraphNode>,
    id_cursor: AbstractSyntaxGraphNodeId
}

impl AbstractSyntaxGraph {
    pub fn get_root(&self) -> Option<&AbstractSyntaxGraphNode> {
        self.get_node(AbstractSyntaxGraphNodeId::root())
    }

    pub fn get_children(&self, node: &AbstractSyntaxGraphNode) -> Vec<&AbstractSyntaxGraphNode> {
        let mut children = vec!();
        for child_id in &node.children {
            if let Some(child_node) = self.get_node(*child_id) {
                children.push(child_node);
            }
        }
        children
    }

    pub fn get_parent(&self, id: AbstractSyntaxGraphNodeId) -> AbstractSyntaxGraphNodeId {
        if let Some(node) = self.get_node(id) {
            return node.parent;
        }
        AbstractSyntaxGraphNodeId::default()
    }

    pub fn get_node(&self, id: AbstractSyntaxGraphNodeId) -> Option<&AbstractSyntaxGraphNode> {
        match Option::<usize>::from(id) {
            Some(index) => self.nodes.get(index),
            None => None,
        }   
    }

    pub fn get_node_mut(&mut self, id: AbstractSyntaxGraphNodeId) -> Option<&mut AbstractSyntaxGraphNode> {
        match Option::<usize>::from(id) {
            Some(index) => self.nodes.get_mut(index),
            None => None,
        }
    }

    pub fn add_root(&mut self) -> AbstractSyntaxGraphNodeId {
        self.add_node(AbstractSyntaxGraphNode::root())
    }

    pub fn add_child_node(
        &mut self,
        parent: AbstractSyntaxGraphNodeId,
        node_type: AbstractSyntaxControlType
    ) -> AbstractSyntaxGraphNodeId {
        let node = AbstractSyntaxGraphNode::new(node_type, parent);
        let id = self.add_node(node);
        if let Some(parent_node) = self.get_node_mut(parent) {
            parent_node.children.push(id);
        }        
        id
    }

    pub fn add_node_property(&mut self, node_id: AbstractSyntaxGraphNodeId, property: AbstractSyntaxProperty) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.add_property(property);
        }
    }
    
    fn add_node(&mut self, node: AbstractSyntaxGraphNode) -> AbstractSyntaxGraphNodeId {
        self.id_cursor = self.id_cursor.next();
        self.nodes.push(node);
        self.id_cursor
    }
}

impl Default for AbstractSyntaxGraph {
    fn default() -> Self {
        Self {
            nodes: vec!(),
            id_cursor: AbstractSyntaxGraphNodeId::default()
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct AbstractSyntaxGraphNodeId(usize);

impl AbstractSyntaxGraphNodeId {
    pub fn root() -> Self {
        Self(1)
    }

    pub fn next(&mut self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<AbstractSyntaxGraphNodeId> for Option<usize> {
    fn from(from: AbstractSyntaxGraphNodeId) -> Self {
        if from.0 == 0 {
            return None
        }
        Some(from.0 - 1)
    }
}

#[derive(Debug)]
pub struct AbstractSyntaxGraphNode {
    node_type: AbstractSyntaxControlType,
    parent: AbstractSyntaxGraphNodeId,
    children: Vec::<AbstractSyntaxGraphNodeId>,
    properties: Vec::<AbstractSyntaxProperty>
}

impl AbstractSyntaxGraphNode {
    pub fn root() -> Self {
        Self {
            node_type: AbstractSyntaxControlType::Root,
            parent: AbstractSyntaxGraphNodeId::default(),
            children: vec!(),
            properties: vec!()
        }
    }

    pub fn new(from: AbstractSyntaxControlType, parent: AbstractSyntaxGraphNodeId) -> Self {
        Self {
            node_type: from,
            parent,
            children: vec!(),
            properties: vec!()
        }
    }

    pub fn node_type(&self) -> AbstractSyntaxControlType {
        self.node_type
    }

    pub fn properties(&self) -> &Vec<AbstractSyntaxProperty> {
        &self.properties
    }

    pub fn add_property(&mut self, property: AbstractSyntaxProperty) {
        self.properties.push(property)
    }
}