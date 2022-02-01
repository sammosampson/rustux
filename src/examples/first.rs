use crate::prelude::*;

// #[actions]
#[derive(Debug)]
pub enum Actions {
    SelectItem(usize)
}

// #[state]
#[derive(Debug, Default)]
pub struct SelectedClickState {
    selected: Option<usize>
}

// #[reducer]
impl SelectedClickState {
    fn process(&self, action: Actions) -> Self {
        match action {
            Actions::SelectItem(id) => Self{ selected: Some(id) },
        }
    }
}

// #[selector]
pub fn is_selected(state: &SelectedClickState, item_id: usize) -> bool {
    state.selected == Some(item_id)
}

//---------------------------------------------
// derived code from here down
impl Actions {
    pub fn register(ctx: &mut StateContext) {
        ctx.actions_mut().register_action(SelectItemActionContainer::default());
    }
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

    fn run(&self, state: &mut State, arguments: &Vec<SourceTokenPropertyValue>) {
        let action = Actions::SelectItem(collect_properties_usize(arguments, 0).unwrap());
        println!("Running action {:?}", action);
        state.process(1, Box::new(| local_state: &SelectedClickState | local_state.process(action)));
    }
}