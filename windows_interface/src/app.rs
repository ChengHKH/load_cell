use crate::ui;
use native_windows_gui as nwg;
use serialport;

#[derive(Default)]
pub struct App {
    pub window: nwg::Window,
}

impl App {
    pub fn say_goodbye(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye"));
        nwg::stop_thread_dispatch();
    }
}