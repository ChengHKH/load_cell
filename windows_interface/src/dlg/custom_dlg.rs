use winsafe::{prelude::*};

use super::ui;

pub fn drop_down(
    parent: &impl GuiParent,
    title: &str,
    header: &str,
    body: &str,
    list: Vec<String>,
    button_text_one: Option<&str>,
    button_text_two: Option<&str>,
) -> Option<serialport::SerialPortInfo> {
    ui::DlgDropDown::new(
        parent,
        title,
        header,
        body,
        list,
        button_text_one,
        button_text_two,
    ).show()
}