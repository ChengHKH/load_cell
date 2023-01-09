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

pub fn dlg_select_port(parent: &impl GuiParent) -> DlgSelectPort {
    
}