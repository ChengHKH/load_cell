use winsafe::{prelude::*, gui, co};
use serialport;

#[derive(Clone)]
pub struct App {
    window: gui::WindowMain,
    
}

impl App {
    pub fn new() -> App {
        let menu = Self::build_menu().unwrap();

        let window = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "Load Cell Reader".to_owned(),
                size: winsafe::SIZE::new(300, 150),
                menu,
                ..Default::default()
            }
        );

        let new_self = Self {window};
        new_self.events();
        new_self
    }

    fn build_menu() -> winsafe::AnyResult<winsafe::HMENU> {
        let file_submenu = winsafe::HMENU::CreatePopupMenu()?;

        file_submenu.AppendMenuEnum(&[
            winsafe::MenuEnum::Entry(1000, "&New\tCtrl+N"),
            winsafe::MenuEnum::Separator,
            winsafe::MenuEnum::Entry(co::DLGID::CANCEL.into(), "&Exit")
        ])?;

        let main_menu = winsafe::HMENU::CreateMenu()?;

        main_menu.AppendMenuEnum(&[
            winsafe::MenuEnum::Submenu(file_submenu, "&File")
        ])?;

        Ok(main_menu)
    }

    pub fn run(&self) -> gui::MsgResult<i32> {
        self.window.run_main(None)
    }

    fn events(&self) {
        
    }
}

