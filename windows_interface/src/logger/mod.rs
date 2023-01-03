use winsafe::{prelude::*, gui, co, msg};

mod events;
mod ids;
mod ui;

#[derive(Clone)]
struct Logger {
    window: gui::WindowMain,
}

impl Logger {
    pub fn new() -> Self {
        let window = ui::build_logger();
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