use crate::prelude::*;

pub enum Actions {
    SelectItem(u16)
}

pub fn register_actions(ctx: &mut StateContext) {
    let module_path= module_path!();
    ctx.register_action(ActionsSelectItemActionContainer::default(), | arg_0 | Actions::SelectItem(arg_0));
}

pub struct ActionsSelectItemActionContainer {
    path: String
}

impl Default for ActionsSelectItemActionContainer {
    fn default() -> Self {
        Self { path: format!("{}::select_item", module_path!()) }
    }
}