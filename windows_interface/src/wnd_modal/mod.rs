use winsafe::{prelude::*, gui};

use super::logger::Logger;

mod events;
mod funcs;
mod ui;

#[derive(Clone)]
pub struct WndModal {
    window: gui::WindowModal,
    logger: Logger,
}

impl WndModal {
    pub fn new(parent: &impl GuiParent) -> WndModal {
        let new_self = ui::build(parent);
        new_self.events();
        new_self
    }
}