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
    items: Vec<usize>
}

// #[reducer]
impl SelectedClickState {
    fn process(&self, action: Actions) -> Self {
        match action {
            Actions::SelectItem(id) => Self { 
                selected: Some(id), 
                items: [&vec!(id)[..], &self.items[..]].concat()
            },
        }
    }
}

// #[selector]
pub fn is_selected(state: &mut State, item_id: usize) -> bool {
    state.get_local::<SelectedClickState>().selected == Some(item_id)
}

// #[selector]
pub fn get_names(state: &mut State) -> Vec<String> {
    state.get_local::<SelectedClickState>()
        .items
        .iter()
        .map(|id|format!("selected_{}", id))
        .collect()
}

// #[selector]
pub fn get_items(state: &mut State) -> Vec<SelectedItem> {
    state.get_local::<SelectedClickState>()
        .items
        .iter()
        .map(|id| 
            SelectedItem { 
                text: format!("selected_{}", id),
                colour: Colour { r: 255 - (id * 10) as u8 , g: (id * 20) as u8, b: (id * 30) as u8, a: 255 }
            }
        )
        .collect()
}

// #[data_item]
pub struct SelectedItem {
    pub text: String,
    pub colour: Colour
}

//---------------------------------------------
// derived code from here down
pub fn register(ctx: &mut DataContext) {
    ctx.actions_mut().register_action(SelectItemActionContainer::default());
    ctx.selectors_mut().register_selector(IsSelectedSelectorContainer::default());
    ctx.selectors_mut().register_selector(GetNamesSelectorContainer::default());
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
        state.process(Box::new(| local_state: &SelectedClickState | local_state.process(action)));
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

    fn run(&self, _data_arrays: &mut DataArrays, state: &mut State, arguments: &Vec<AbstractSyntaxPropertyValue>) -> Result<AbstractSyntaxPropertyValue, ContainerRunError> {
        if arguments.len() != 2 {
            return Err(ContainerRunError::IncorrectAmountOfArgumentsPassed);
        }

        if !arguments[0].is_state_variable() {
            return Err(ContainerRunError::FirstArgumentNotStateVariable);
        }

        Ok(AbstractSyntaxPropertyValue::Bool(is_selected(state, arguments[1].get_usize_value()?)))
    }
}

pub struct GetNamesSelectorContainer {
    path: String
}

impl Default for GetNamesSelectorContainer {
    fn default() -> Self {
        Self { path: format!("{}::get_names", module_path!()) }
    }
}

impl SelectorContainer for GetNamesSelectorContainer {
    fn function_name(&self) -> &str {
        &self.path
    }

    fn run(&self, data_arrays: &mut DataArrays, state: &mut State, arguments: &Vec<AbstractSyntaxPropertyValue>) -> Result<AbstractSyntaxPropertyValue, ContainerRunError> {
        if arguments.len() != 1 {
            return Err(ContainerRunError::IncorrectAmountOfArgumentsPassed);
        }

        if !arguments[0].is_state_variable() {
            return Err(ContainerRunError::FirstArgumentNotStateVariable);
        }

        let items = get_names(state);
        let data_id = data_arrays.add_string_array(items);

        Ok(AbstractSyntaxPropertyValue::DataArray(data_id, 0))
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

    fn run(&self, data_arrays: &mut DataArrays, state: &mut State, arguments: &Vec<AbstractSyntaxPropertyValue>) -> Result<AbstractSyntaxPropertyValue, ContainerRunError> {
        if arguments.len() != 1 {
            return Err(ContainerRunError::IncorrectAmountOfArgumentsPassed);
        }

        if !arguments[0].is_state_variable() {
            return Err(ContainerRunError::FirstArgumentNotStateVariable);
        }

        let items = get_items(state);
        let data_id = data_arrays.add(SelectedItemDataArray::from(items));

        Ok(AbstractSyntaxPropertyValue::DataArray(data_id, 0))
    }
}

pub struct SelectedItemDataArray(Vec<SelectedItem>);

impl DataArray for SelectedItemDataArray {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn get_array_item_value(&self, position: usize, variable: &VariablePath) -> Option<AbstractSyntaxPropertyValue> {
        if let Some(value) = self.0.get(position) {
            if let Some(variable_property) = variable.property_part() {
                if variable_property == "text" {
                    return Some(AbstractSyntaxPropertyValue::String(value.text.clone()))   
                }
                if variable_property == "colour" {
                    return Some(AbstractSyntaxPropertyValue::Colour(value.colour.clone()))   
                }
            }
        }
        None
    }
}

impl From<Vec<SelectedItem>> for SelectedItemDataArray {
    fn from(from: Vec<SelectedItem>) -> Self {
        Self(from)
    }
}