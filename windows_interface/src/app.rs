use crate::ui;

use winsafe::{prelude::*, gui};
use serialport;

#[derive(Clone)]
pub struct App {
    window: gui::WindowMain,
}

impl App {
    pub fn new() -> App {
        let menu = ui::build_menu().unwrap();

        let window = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "Load Cell Reader".to_owned(),
                size: winsafe::SIZE::new(300, 150),
                menu,
                ..Default::default()
            }
        );

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