use winsafe::{prelude::*, co};

use super::{Reader, ui, funcs};

impl Reader {
    pub(super) fn events(&self) {
        self.window.on().wm_create({
            let reader_window = self.window.clone();
            move |_| {
                reader_window.hwnd().SetTimer(1, 1000, None)?;
                Ok(0)
            }
        });

        self.window.on().wm_paint({
            let reader_window = self.window.clone();
            move || {
                ui::draw_reading(reader_window.hwnd(), funcs::get_reading())?;
                Ok(())
            }
        });

        self.window.on().wm_timer(1, {
            let reader_window = self.window.clone();
            move || {
                reader_window.hwnd().RedrawWindow(
                    &reader_window.hwnd().GetClientRect()?,
                    Handle::NULL,
                    co::RDW::INVALIDATE | co::RDW::UPDATENOW,
                )?;
                Ok(())
            }
        });
    }
}