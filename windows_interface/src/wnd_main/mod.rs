use winsafe::{prelude::*, gui};

use super::reader::Reader;

mod events;
mod funcs;
mod ids;
mod ui;

#[derive(Clone)]
pub struct WndMain {
    window: gui::WindowMain,
    reader: Reader,
}

impl WndMain {
    pub fn new() -> WndMain {
        let new_self = ui::build();
        new_self.events();
        new_self
    }

    pub fn run(&self) -> gui::MsgResult<i32> {
        self.window.run_main(None)
    }
}