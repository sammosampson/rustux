use crate::prelude::*;

pub enum Actions {
    SelectItem(u16)
}

pub fn register_actions(ctx: &mut StateContext) {
    ctx.register_action(ActionsSelectItemActionContainer::default());
}

pub struct ActionsSelectItemActionContainer {
    path: String
}

impl Default for ActionsSelectItemActionContainer {
    fn default() -> Self {
        Self { path: format!("{}::select_item", module_path!()) }
    }
}

impl ActionContainer for ActionsSelectItemActionContainer {
    fn path(&self) -> &str {
        &self.path
    }
}