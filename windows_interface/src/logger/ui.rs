impl Logger {
    fn build() -> gui::WindowMain {
        let menu = build_logger_menu().unwrap();
    
        gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "Load Cell Logger".to_owned(),
                size: SIZE::new(600, 300),
                menu,
                ..Default::default()
            }
        )
    }
    
    fn build_logger_menu() -> winsafe::AnyResult<winsafe::HMENU> {
        let file_submenu = winsafe::HMENU::CreatePopupMenu()?;
        let options_submenu = winsafe::HMENU::CreatePopupMenu()?;
    
        file_submenu.AppendMenuEnum(&[
            winsafe::MenuEnum::Entry(ids::FILE_NEW, "&New Log File\tCtrl+N"),
            winsafe::MenuEnum::Entry(ids::SAVE, "Save\tCtrl+S"),
            winsafe::MenuEnum::Entry(ids::SAVE_AS, "Save As"),
            winsafe::MenuEnum::Separator,
            winsafe::MenuEnum::Entry(co::DLGID::CANCEL.into(), "&Exit\tAlt+F4"),
        ])?;
    
        options_submenu.AppendMenuEnum(&[
            winsafe::MenuEnum::Entry(ids::OPTIONS_TARE, "&Tare"),
            winsafe::MenuEnum::Entry(ids::OPTIONS_RECALIBRATE, "&Recalibrate"),
            winsafe::MenuEnum::Separator,
            winsafe::MenuEnum::Entry(ids::OPTIONS_KG, "&kg"),
            winsafe::MenuEnum::Entry(ids::OPTIONS_G, "&g"),
            winsafe::MenuEnum::Entry(ids::OPTIONS_LB, "&lb"),
            winsafe::MenuEnum::Entry(ids::OPTIONS_N, "&N"),
        ]
        )?;
    
        let logger_menu = winsafe::HMENU::CreateMenu()?;
    
        logger_menu.AppendMenuEnum(&[
            winsafe::MenuEnum::Submenu(file_submenu, "&File"),
            winsafe::MenuEnum::Submenu(options_submenu, "&Options"),
        ])?;
    
        Ok(logger_menu)
    }
}