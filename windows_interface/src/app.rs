use crate::ui;

use winsafe::{prelude::*, gui};
use serialport;

#[derive(Clone)]
pub struct App {
    window: gui::WindowMain,
}

impl App {
    pub fn new() -> App {
        let window = ui::build_window();
        let new_self = Self {window};
        new_self.events();
        new_self
    }

    pub fn run(&self) -> gui::MsgResult<i32> {
        self.window.run_main(None)
    }

    fn events(&self) {
        
    }
}

#[cfg(tests)]
mod tests {

}