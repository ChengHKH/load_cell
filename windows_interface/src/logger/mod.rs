use winsafe::{prelude::*, gui, co, msg};

mod ui;
mod events;

#[derive(Clone)]
pub struct Logger {
    window: gui::WindowControl,
}

impl Logger {
    pub fn new(parent: &impl GuiParent) -> Logger {
        let new_self = ui::build(parent);
        new_self.events();
        new_self
    }
}