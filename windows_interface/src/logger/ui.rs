use winsafe::{prelude::*, gui, SIZE, POINT};

use super::Logger;


pub(super) fn build(parent: &impl GuiParent) -> Logger {
    let window = gui::WindowControl::new(
        parent,
        gui::WindowControlOpts {
            position: POINT::new(0, 0),
            size: SIZE::new(360, 130),
            ..Default::default()
        }
    );

    Logger {window}
}