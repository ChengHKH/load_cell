use winsafe::{prelude::*, gui};

mod events;
mod funcs;
mod ui;

#[derive(Clone)]
pub struct Reader {
    window: gui::WindowControl,
}

impl Reader {
    pub fn new(parent: &impl GuiParent) -> Reader {
        let new_self = ui::build(parent);
        new_self.events();
        new_self
    }
}
