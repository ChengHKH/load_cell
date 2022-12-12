use std::{rc::Rc, cell::RefCell};

use crate::{ids, ui};

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
            let main_window = self.window.clone(); 
            move |_| {
                let ports = get_ports();
                match ports {
                    Some(info) => {
                        let port = DLGSomePorts::new(&main_window, info).show();
                        match port {
                            Some(i) => {
                                open_port(i);
                            },
                            None => DLGNotConnected::new(&main_window).show(),
                        }
                    },
                    None => DLGNoPorts::new(&main_window).show(),
                };
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
struct DLGNotConnected {
    window: gui::WindowModal,
    text: gui::Label,
    btn_ok: gui::Button,
}

impl DLGNotConnected {
    pub fn new(parent: &impl GuiParent) -> Self {
        let window = ui::build_modal_error(parent);
        let text = ui::text_not_connected(&window);
        let btn_ok = ui::build_modal_ok(&window, "error", 1);
        let new_self = Self {window, text, btn_ok};
        new_self.events();
        new_self
    }

    pub fn show(&self) {
        self.window.show_modal();
    }

    fn events(&self) {

    }
}

#[derive(Clone)]
struct DLGNoPorts {
    window: gui::WindowModal,
    text: gui::Label,
    btn_ok: gui::Button,
}

impl DLGNoPorts {
    pub fn new(parent: &impl GuiParent) -> Self {
        let window = ui::build_modal_error(parent);
        let text = ui::text_no_ports(&window);
        let btn_ok = ui::build_modal_ok(&window, "error", 1);
        let new_self = Self {window, text, btn_ok};
        new_self.events();
        new_self
    }

    pub fn show(&self) {
        self.window.show_modal();
    }

    fn events(&self) {

    }
}

#[derive(Clone)]
struct DLGSomePorts {
    window: gui::WindowModal,
    ports_list: gui::ComboBox,
    btn_ok: gui::Button,
    btn_cancel: gui::Button,
    
    return_value: Rc<RefCell<Option<serialport::SerialPortInfo>>>,
}

impl DLGSomePorts {
    pub fn new(parent: &impl GuiParent, ports: Vec<serialport::SerialPortInfo>) -> Self {
        let window = ui::build_modal(parent);
        let ports_list = ui::build_ports_list(&window, ports);
        let btn_ok = ui::build_modal_ok(&window, "input", 2);
        let btn_cancel = ui::build_modal_cancel(&window, "input", 1);
        
        let new_self = Self {
            window,
            ports_list,
            btn_ok,
            btn_cancel,
            return_value: Rc::new(RefCell::new(None)),
        };

        new_self.events();
        new_self
    }

    pub fn show(&self) -> Option<serialport::SerialPortInfo> {
        self.window.show_modal();
        self.return_value.borrow().as_ref().map(|info| info.clone())
    }

    fn events(&self) {
        // self.btn_ok.on().bn_clicked({
            
        // });
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
                ui::draw_reading(reader_window.hwnd(), get_reading())?;
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

fn get_ports() -> Option<Vec<serialport::SerialPortInfo>> {
    println!("Finding serial ports...");
    let ports = serialport::available_ports().expect("No ports found!");
    let mut arduino_ports = Vec::new();
    
    for p in ports {
        if let SerialPortType::UsbPort(ref i) = p.port_type {
            if i.vid == 9025 || i.vid == 10755 {
                arduino_ports.push(p);
            }
        }
    };

    if arduino_ports.len() == 0 {
        None
    } else {
        Some(arduino_ports)
    }
}

fn open_port(port: serialport::SerialPortInfo) -> serialport::Result<Box<dyn serialport::SerialPort>> {
    serialport::new(port.port_name, 115200).open()
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