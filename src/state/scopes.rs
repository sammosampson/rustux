use crate::prelude::*;

#[derive(Default)]
pub struct DataScopes {
    inner: HashMap<AbstractSyntaxGraphNodeId, DataScope>,
    current: Vec<AbstractSyntaxGraphNodeId>
}

impl DataScopes {
    pub fn set(&mut self, node_id: AbstractSyntaxGraphNodeId) {
        self.inner.insert(node_id, DataScope::default());
        self.push(node_id);
    }

    pub fn push(&mut self, node_id: AbstractSyntaxGraphNodeId) {
        self.current.push(node_id);
    }

    pub fn pop(&mut self) {
        self.current.pop();
    }

    pub fn current_mut(&mut self) -> &mut DataScope {
        self.inner.get_mut(self.current.last().unwrap()).unwrap()
        
    }
}

#[derive(Default)]
pub struct DataScope {
    state: State,
    variables: Variables
}

impl DataScope {
    pub fn set_control_args(&mut self, arguments: ControlArguments) {
        self.variables = arguments.into();
    }

    pub fn variables_mut(&mut self) -> &mut Variables {
        &mut self.variables
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
}