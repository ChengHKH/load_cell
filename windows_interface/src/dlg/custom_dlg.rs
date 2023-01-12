use winsafe::{prelude::*};

use super::ui;

pub fn drop_down(
    parent: &impl GuiParent,
    title: &str,
    header: &str,
    body: &str,
    list: Vec<String>,
    button_text_one: Option<&str>,
) -> Option<String> {
    ui::DlgDropDown::new(
        parent,
        title,
        header,
        body,
        list,
        button_text_one,
    ).show()
}