use std::{rc::Rc, cell::RefCell};

use crate::{ids, ui, main};

use winsafe::{prelude::*, gui, co};
use serialport::{SerialPortType, SerialPortInfo};

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
        #[cfg(not(test))]
        self.window.on().wm_create({
            let main_window = self.window.clone(); 
            move |_| {
                let ports = get_ports();
                
                let port = match ports {
                    Some(info) => {
                        let names = get_port_names(info);
                        DlgSelectPort::new(&main_window, names).show()
                    },
                    None => {
                        ui::dlg_no_ports(&main_window);
                        return Ok(0)
                    },
                };

                let data = match port {
                    Some(p) => open_port(p),
                    None => {
                        ui::dlg_not_connected(&main_window);
                        return Ok(0);
                    },
                };
                
                Ok(0)
           }
        });

        #[cfg(test)]
        self.window.on().wm_create({
            let main_window = self.window.clone(); 
            move |_| {
                let names = vec!["Test".to_owned(), "Test".to_owned()];
                DlgSelectPort::new(&main_window, names).show();
                
                Ok(0)
           }
        });

        self.window.on().wm_command_accel_menu(ids::FILE_NEW, {
            let main_window = self.window.clone(); 
            move || {
                let _logger = Logger::new().run()?;
                Ok(())
            }
        });

    }
}

#[derive(Clone)]
struct DlgSelectPort {
    window: gui::WindowModal,
    main_instruction: gui::Label,
    secondary_instruction: gui::Label,
    ports_list: gui::ComboBox,
    btn_ok: gui::Button,
    btn_cancel: gui::Button,
    
    return_value: Rc<RefCell<Option<serialport::SerialPortInfo>>>,
}

impl DlgSelectPort {
    pub fn new(parent: &impl GuiParent, names: Vec<String>) -> Self {
        let window = ui::build_select_port(parent);
        let main_instruction = ui::build_select_port_instruction_main(&window);
        let secondary_instruction = ui::build_select_port_instruction_secondary(&window);
        let ports_list = ui::build_select_port_list(&window, names);
        let btn_ok = ui::build_select_port_connect(&window, "input", 2);
        let btn_cancel = ui::build_select_port_cancel(&window, "input", 1);
        
        let new_self = Self {
            window,
            main_instruction,
            secondary_instruction,
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
        self.window.on().wm_create({
            let main_instruction = self.main_instruction.clone();
            move |_| {
                let hwnd = main_instruction.hwnd();
                ui::draw_instruction_main_font(hwnd)?;
                Ok(0)
            }
        });

        // self.window.on().wm_ctl_color_static({
        //     let main_instruction = self.main_instruction.clone();
        //     move |_| {
        //         let color = ui::draw_instruction_main_color(&main_instruction)?;
        //         Ok(color)
        //     }
        // });
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
}

impl Reader {
    pub fn new(parent: &impl GuiParent) -> Self {
        let window = ui::build_reader(parent);

        let new_self = Self {
            window,
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
    let value = "1234";

    let unit = " g";

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

fn get_port_names(info: Vec<SerialPortInfo>) -> Vec<String> {
    let mut port_names = Vec::new();
    for i in info {
        if let serialport::SerialPortType::UsbPort(usb) = i.port_type {
            let name = usb.product.unwrap_or_else(
                || usb.manufacturer.unwrap_or_else(
                    || String::from("Unknown")) + "USB");

            port_names.push(name);
        }
    };
    port_names
}

fn open_port(info: SerialPortInfo) -> serialport::Result<Box<dyn serialport::SerialPort>> {
        serialport::new(info.port_name, 115200).open()
}