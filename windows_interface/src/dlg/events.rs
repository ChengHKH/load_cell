use winsafe::{prelude::*};

use super::ui;

impl ui::DlgDropDown {
    pub fn events(&self) {
        self.dialog.buttons.button_one.unwrap().on().bn_clicked({
            let self2 = self.clone();
            move || {
                *self2.return_value.try_borrow_mut()? = Some(self2.drop_down.text());

        }});
    }
}