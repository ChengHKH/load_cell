use winsafe::{prelude::*, co};

use super::ui;

impl ui::DlgDropDown {
    pub fn events(&self) {
        self.dialog.buttons.button_one.unwrap().on().bn_clicked({
            let self2 = self.clone();
            move || {
                *self2.return_value.try_borrow_mut()? = Some(self2.drop_down.text());
                self2.dialog.window.hwnd().EndDialog(0)?;
                Ok(())
            }
        });

        self.dialog.buttons.button_two.unwrap().on().bn_clicked({
            let self2 = self.clone();
            move || {
                *self2.return_value.try_borrow_mut()? = None;
                self2.dialog.window.hwnd().EndDialog(0)?;
                Ok(())
            }
        });

        self.dialog.window.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
            let self2 = self.clone();
            move || {
                *self2.return_value.try_borrow_mut()? = None;
                self2.dialog.window.hwnd().EndDialog(0)?;
                Ok(())
            }
        });
    }
}