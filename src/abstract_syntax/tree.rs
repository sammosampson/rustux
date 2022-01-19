use crate::prelude::*;

pub struct AbstractSyntaxTree {
    nodes: Vec<AbstractSyntaxTreeNode>,
    id_cursor: AbstractSyntaxTreeNodeId
}

impl AbstractSyntaxTree {
    pub fn get_root(&self) -> Option<&AbstractSyntaxTreeNode> {
        self.get_node(AbstractSyntaxTreeNodeId::root())
    }

    pub fn get_children(&self, node: &AbstractSyntaxTreeNode) -> Vec<&AbstractSyntaxTreeNode> {
        let mut children = vec!();
        for child_id in &node.children {
            if let Some(child_node) = self.get_node(*child_id) {
                children.push(child_node);
            }
        }
        children
    }

    pub fn get_parent(&self, id: AbstractSyntaxTreeNodeId) -> AbstractSyntaxTreeNodeId {
        if let Some(node) = self.get_node(id) {
            return node.parent;
        }
        AbstractSyntaxTreeNodeId::default()
    }

    pub fn get_node(&self, id: AbstractSyntaxTreeNodeId) -> Option<&AbstractSyntaxTreeNode> {
        self.nodes.get(*id - 1)
    }

    pub fn get_node_mut(&mut self, id: AbstractSyntaxTreeNodeId) -> Option<&mut AbstractSyntaxTreeNode> {
        self.nodes.get_mut(*id - 1)
    }

    pub fn add_root(&mut self) -> AbstractSyntaxTreeNodeId {
        self.add_node(AbstractSyntaxTreeNode::root())
    }

    pub fn add_child_node(
        &mut self,
        parent: AbstractSyntaxTreeNodeId,
        node_type: AbstractSyntaxTokenType
    ) -> AbstractSyntaxTreeNodeId {
        let id = self.add_node(AbstractSyntaxTreeNode::new(node_type, parent));
        if let Some(parent_node) = self.get_node_mut(parent) {
            parent_node.children.push(id);
        }        
        id
    }

    fn add_node(&mut self, node: AbstractSyntaxTreeNode) -> AbstractSyntaxTreeNodeId {
        self.id_cursor = self.id_cursor.next();
        self.nodes.push(node);
        self.id_cursor
    }
}

impl Default for AbstractSyntaxTree {
    fn default() -> Self {
        Self {
            nodes: vec!(),
            id_cursor: AbstractSyntaxTreeNodeId::default()
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct AbstractSyntaxTreeNodeId(usize);

impl AbstractSyntaxTreeNodeId {
    pub fn root() -> Self {
        Self(1)
    }

    pub fn next(&mut self) -> Self {
        Self(self.0 + 1)
    }
}

impl Deref for AbstractSyntaxTreeNodeId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct AbstractSyntaxTreeNode {
    node_type: AbstractSyntaxTokenType,
    parent: AbstractSyntaxTreeNodeId,
    children: Vec::<AbstractSyntaxTreeNodeId>
}

impl AbstractSyntaxTreeNode {
    pub fn root() -> Self {
        Self {
            node_type: AbstractSyntaxTokenType::Root,
            parent: AbstractSyntaxTreeNodeId::default(),
            children: vec!()
        }
    }

    pub fn new(from: AbstractSyntaxTokenType, parent: AbstractSyntaxTreeNodeId) -> Self {
        Self {
            node_type: from,
            parent,
            children: vec!()
        }
    }

    pub fn node_type(&self) -> AbstractSyntaxTokenType {
        self.node_type
    }
}

#[derive(Default)]
pub struct AbstractSyntaxTokenStreamLinker {
    ast: AbstractSyntaxTree,
    current_node: AbstractSyntaxTreeNodeId
}

impl AbstractSyntaxTokenStreamLinker {
    pub fn ast(self) -> AbstractSyntaxTree {
        self.ast
    }
}

impl AbstractSyntaxTokenStreamVisitor for AbstractSyntaxTokenStreamLinker {
    fn token_error(&mut self, error: &AbstractSyntaxTokenError) {
        todo!()
    }

    fn start_node(&mut self, node_type: &AbstractSyntaxTokenType) {
        match node_type {
            AbstractSyntaxTokenType::Root =>
                self.current_node = self.ast.add_root(),
            node_type => 
                self.current_node = self.ast.add_child_node(self.current_node, *node_type),
        }
    }

    fn end_node(&mut self, node_type: &AbstractSyntaxTokenType) {
        self.current_node = self.ast.get_parent(self.current_node);
    }
}