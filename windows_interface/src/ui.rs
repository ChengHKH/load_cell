use std::{rc::Rc, cell::RefCell, ops::Deref};

use crate::app::App;
use native_windows_gui as nwg;
use nwg::Event;

pub struct Ui {
    inner: Rc<App>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl nwg::NativeUi<Ui> for App {
    fn build_ui(mut data: App) -> Result<Ui, nwg::NwgError> {
        nwg::Window::builder()
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .size((300, 135))
            .position((300, 300))
            .title("Load Cell Reader")
            .build(&mut data.window)?;

        let ui = Ui {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                match evt {
                    Event::OnWindowClose =>
                        if &handle == &ui.window {
                            App::say_goodbye(&ui);
                        },
                    _ => {}
                }
            }
        };

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));

        return Ok(ui);
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for Ui {
    type Target = App;

    fn deref(&self) -> &App {
        &self.inner
    }
}