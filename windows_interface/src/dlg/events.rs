use winsafe::{prelude::*, co, msg};

use super::ui;

impl ui::DlgDropDown {
    pub fn events(&self) {
        self.connect_button.on().bn_clicked({
            let self2 = self.clone();
            move || {
                *self2.return_value.try_borrow_mut()? = Some(self2.drop_down.text());
                self2.window.hwnd().SendMessage(msg::wm::Close {});
                Ok(())
            }
        });

        self.cancel_button.on().bn_clicked({
            let self2 = self.clone();
            move || {
                *self2.return_value.try_borrow_mut()? = None;
                self2.window.hwnd().SendMessage(msg::wm::Close {});
                Ok(())
            }
        });

        self.window.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
            let self2 = self.clone();
            move || {
                *self2.return_value.try_borrow_mut()? = None;
                self2.window.hwnd().SendMessage(msg::wm::Close {});
                Ok(())
            }
        });
    }
}