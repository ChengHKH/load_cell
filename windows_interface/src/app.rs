use crate::{ids, ui::{self, draw_reading}};

use winsafe::{prelude::*, gui, co};
use serialport::{self, SerialPortType};

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
        self.window.on().wm_create({
           move |_| {
                // get_ports();
                Ok(0)
           } 
        });

        self.window.on().wm_command_accel_menu(ids::FILE_NEW, {
            move || {
                let _logger = Logger::new().run()?;
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
    // reading: gui::Label,
}

impl Reader {
    pub fn new(parent: &impl GuiParent) -> Self {
        let window = ui::build_reader(parent);
        // let reading = ui::build_reading(&window);

        let new_self = Self {
            window,
            // reading,
        };
        new_self.events();
        new_self
    }

    fn events(&self) {
        self.window.on().wm_create({
            let reader_window = self.window.clone();
            move |_| {
                reader_window.hwnd().SetTimer(1, 1000, None)?;
                Ok(0)
            }
        });

        self.window.on().wm_paint({
            let reader_window = self.window.clone();
            move || {
                draw_reading(reader_window.hwnd(), get_reading())?;
                Ok(())
            }
        });

        self.window.on().wm_timer(1, {
            let reader_window = self.window.clone();
            move || {
                reader_window.hwnd().RedrawWindow(
                    &reader_window.hwnd().GetClientRect()?,
                    Handle::NULL,
                    co::RDW::INVALIDATE | co::RDW::UPDATENOW,
                )?;
                Ok(())
            }
        });
    }
}

fn get_reading<'a>() -> [&'a str; 2] {
    let value = "1.23";

    let unit = " kg";

    [value, unit]
}

pub fn get_ports() -> Option<Vec<serialport::UsbPortInfo>> {
    println!("Finding serial ports...");
    let ports = serialport::available_ports().expect("No ports found!");
    let mut arduino_ports = Vec::new();
    
    for p in ports {
        if let SerialPortType::UsbPort(i) = p.port_type {
            if i.vid == 9025 || i.vid == 10755 {
                arduino_ports.push(i);
            }
        }
    };

    if arduino_ports.len() == 0 {
        None
    } else {
        Some(arduino_ports)
    }
}

// let port = serialport::new(p.port_name, 115200)
//                     .open();
            
//                 match port {
//                     Ok(mut port) => {
//                         println!("Success! Found arduino port!")
//                     }
//                     Err(e) => {
//                         eprintln!("Failed!")
//                     }
//                 }

#[cfg(tests)]
mod tests {
    
}