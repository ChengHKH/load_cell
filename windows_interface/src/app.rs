use crate::{ids, ui};

use winsafe::{prelude::*, gui};
use serialport;

#[derive(Clone)]
pub struct App {
    window: gui::WindowMain,
}

impl App {
    pub fn new() -> App {
        let window = ui::build_main();
        let new_self = Self {window};
        new_self.events();
        new_self
    }

    pub fn run(&self) -> gui::MsgResult<i32> {
        self.window.run_main(None)
    }

    fn events(&self) {
        self.window.on().wm_command_accel_menu(ids::FILE_NEW, {
            let main_window = self.window.clone();
            move || {
                let logger = Logger::new().run();
                Ok(())
            }
        });

    }
}


struct Logger {
    window: gui::WindowMain,
}

impl Logger {
    pub fn new() -> Logger {
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

#[cfg(tests)]
mod tests {

}