use winsafe::{prelude::*, gui, co, msg, task_dlg, HBRUSH};

mod ui;

pub fn dlg_not_connected(parent: &impl GuiParent) -> () {
    task_dlg::error(
        parent.hwnd(),
        "Error",
        Some("Connection failed"),
        "Please connect to an Arduino to use this tool.",
    ).unwrap();
}

pub fn dlg_no_ports(parent: &impl GuiParent) -> () {
    task_dlg::error(
        parent.hwnd(),
        "Error",
        Some("No Arduinos found"),
        "Please insert an Arduino to use this tool.",
    ).unwrap();
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
        // self.btn_ok.on().bn_clicked({
            
        // });
    }
}