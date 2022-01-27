use crate::prelude::*;

#[derive(Debug)]
pub enum Actions {
    SelectItem(u16)
}

#[derive(Debug, Default)]
pub struct SelectedClickState {
    selected: Option<u16>
}

impl SelectedClickState {
    fn process(&self, action: Actions) -> Self {
        match action {
            Actions::SelectItem(id) => Self{ selected: Some(id) },
        }
    }
}



impl Actions {
    pub fn register(ctx: &mut StateContext) {
        ctx.register_action(SelectItemActionContainer::default());
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

    fn run(&self, arguments: &Vec<SourceTokenPropertyValue>) {
        let action = Actions::SelectItem(collect_properties_unsigned_int(arguments, 0).unwrap());
        println!("Running action {:?}", action);
        let state = SelectedClickState::default();
        let state = state.process(action);
        println!("State {:?}", state);
    }
}