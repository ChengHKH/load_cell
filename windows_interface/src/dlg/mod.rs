use winsafe::{prelude::*, gui, co, msg, task_dlg, HBRUSH};

mod custom_dlg;
mod events;
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

pub fn dlg_select_port(parent: &impl GuiParent, list: Vec<String>) -> Option<String> {
    custom_dlg::drop_down(
        parent,
        "Connect to Arduino",
        "Connect to Arduino",
        "Please select an Arduino to connect to.",
        list,
        Some("Connect"),
    )
}