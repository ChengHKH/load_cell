use crate::{ids, ui};

use winsafe::{prelude::*, gui};
use serialport;

pub fn list_ports() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}

#[derive(Clone)]
pub struct App {
    window: gui::WindowMain,
    reader: Reader,
}

impl App {
    pub fn new() -> App {
        let window = ui::build_main();
        let reader = Reader::new(&window);
        let new_self = Self {window, reader};
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

#[derive(Clone)]
struct Reader {
    window: gui::WindowControl,
    reading: gui::Label,
}

impl Reader {
    pub fn new(parent: &impl GuiParent) -> Self {
        let window = ui::build_reader(parent);

        let reading = ui::build_reading(&window);

        let new_self = Self {
            window,
            reading,
        };
        new_self.events();
        new_self
    }

    fn events(&self) {

    }
}

#[cfg(tests)]
mod tests {

}