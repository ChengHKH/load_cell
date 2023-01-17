use winsafe::{prelude::*, gui, SIZE};

use super::WndModal;
use super::Logger;

pub(super) fn build(parent: &impl GuiParent) -> WndModal {
    let window = gui::WindowModal::new(
        parent,
        gui::WindowModalOpts {
            title: "Logger".to_owned(),
            size: SIZE::new(500, 300),
            ..Default::default()
        }
    );

    let logger = Logger::new(&window);

    WndModal {window, logger}
}