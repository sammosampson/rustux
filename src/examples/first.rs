use crate::prelude::*;

// #[actions]
#[derive(Debug)]
pub enum Actions {
    SelectItem(usize)
}

// #[state]
#[derive(Debug, Default)]
pub struct SelectedClickState {
    selected: Option<usize>,
    items: Vec<String>
}

// #[reducer]
impl SelectedClickState {
    fn process(&self, action: Actions) -> Self {
        match action {
            Actions::SelectItem(id) => Self{ 
                selected: Some(id), 
                items: [&vec!(format!("selected_{}", id))[..], &self.items[..]].concat()
            },
        }
    }
}

// #[selector]
pub fn is_selected(state: &mut State, item_id: usize) -> bool {
    state.get_local::<SelectedClickState>(1).selected == Some(item_id)
}

// #[selector]
pub fn get_items(state: &mut State) -> &Vec<String> {
    &state.get_local::<SelectedClickState>(1).items
}

//---------------------------------------------
// derived code from here down
pub fn register(ctx: &mut DataContext) {
    ctx.actions_mut().register_action(SelectItemActionContainer::default());
    ctx.selectors_mut().register_selector(IsSelectedSelectorContainer::default());
    ctx.selectors_mut().register_selector(GetItemsSelectorContainer::default());
}

pub struct SelectItemActionContainer {
    path: String
}

impl Default for SelectItemActionContainer {
    fn default() -> Self {
        Self { path: format!("{}::select_item", module_path!()) }
    }
}

impl ActionContainer for SelectItemActionContainer {
    fn function_name(&self) -> &str {
        &self.path
    }

    fn run(&self, state: &mut State, arguments: &Vec<AbstractSyntaxPropertyValue>) -> Result<(), ContainerRunError> {
        if arguments.len() != 1 {
            return Err(ContainerRunError::IncorrectAmountOfArgumentsPassed);
        }

        let action = Actions::SelectItem(arguments[0].get_usize_value()?);
        println!("Running action {:?}", action);
        state.process(1, Box::new(| local_state: &SelectedClickState | local_state.process(action)));
        Ok(())
    }
}

pub struct IsSelectedSelectorContainer {
    path: String
}

impl Default for IsSelectedSelectorContainer {
    fn default() -> Self {
        Self { path: format!("{}::is_selected", module_path!()) }
    }
}

impl SelectorContainer for IsSelectedSelectorContainer {
    fn function_name(&self) -> &str {
        &self.path
    }

    fn run(&self, state: &mut State, arguments: &Vec<AbstractSyntaxPropertyValue>) -> Result<AbstractSyntaxPropertyValue, ContainerRunError> {
        if arguments.len() != 2 {
            return Err(ContainerRunError::IncorrectAmountOfArgumentsPassed);
        }

        if !arguments[0].is_state_variable() {
            return Err(ContainerRunError::FirstArgumentNotStateVariable);
        }

        Ok(AbstractSyntaxPropertyValue::Bool(is_selected(state, arguments[1].get_usize_value()?)))
    }
}

pub struct GetItemsSelectorContainer {
    path: String
}

impl Default for GetItemsSelectorContainer {
    fn default() -> Self {
        Self { path: format!("{}::get_items", module_path!()) }
    }
}

impl SelectorContainer for GetItemsSelectorContainer {
    fn function_name(&self) -> &str {
        &self.path
    }

    fn run(&self, state: &mut State, arguments: &Vec<AbstractSyntaxPropertyValue>) -> Result<AbstractSyntaxPropertyValue, ContainerRunError> {
        if arguments.len() != 1 {
            return Err(ContainerRunError::IncorrectAmountOfArgumentsPassed);
        }

        if !arguments[0].is_state_variable() {
            return Err(ContainerRunError::FirstArgumentNotStateVariable);
        }

        Ok(get_items(state).into())
    }
}