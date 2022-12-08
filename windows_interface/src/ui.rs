use crate::ids;

use winsafe::{prelude::*, co, gui, POINT, SIZE, HWND, PAINTSTRUCT, HFONT};

pub fn build_logger() -> gui::WindowMain {
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

pub fn build_main() -> gui::WindowMain {
    let menu = build_main_menu().unwrap();
    
    gui::WindowMain::new(
        gui::WindowMainOpts {
            title: "Load Cell Reader".to_owned(),
            size: SIZE::new(300, 150),
            menu,
            ..Default::default()
        }
    )
}

fn build_main_menu() -> winsafe::AnyResult<winsafe::HMENU> {
    let file_submenu = winsafe::HMENU::CreatePopupMenu()?;
    let options_submenu = winsafe::HMENU::CreatePopupMenu()?;

    file_submenu.AppendMenuEnum(&[
        winsafe::MenuEnum::Entry(ids::FILE_NEW, "&New Log File\tCtrl+N"),
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

    // options_submenu.CheckMenuRadioItem(
    //     IdPos::Id(ID_OPTIONS_KG),
    //     IdPos::Id(ID_OPTIONS_N),
    //     IdPos::Id(ID_OPTIONS_KG),
    // )?;

    let main_menu = winsafe::HMENU::CreateMenu()?;

    main_menu.AppendMenuEnum(&[
        winsafe::MenuEnum::Submenu(file_submenu, "&File"),
        winsafe::MenuEnum::Submenu(options_submenu, "&Options"),
    ])?;

    Ok(main_menu)
}

pub fn build_reader(parent: &impl GuiParent) -> gui::WindowControl {
    gui::WindowControl::new(
        parent,
        gui::WindowControlOpts {
            position: POINT::new(0, 0),
            size: SIZE::new(300, 150),
            ..Default::default()
        }
    )
}

pub fn draw_reading(hwnd: HWND, reading: [&str; 2]) -> winsafe::AnyResult<()> {
    let mut ps = PAINTSTRUCT::default();
    let hdc = hwnd.BeginPaint(&mut ps)?;
    
    let hfont = HFONT::CreateFont(
        SIZE::new(0, 100),
        0,
        0,
        co::FW::BOLD,
        false,
        false,
        false,
        co::CHARSET::DEFAULT,
        co::OUT_PRECIS::DEFAULT,
        co::CLIP::DEFAULT_PRECIS,
        co::QUALITY::DEFAULT,
        co::PITCH::DEFAULT,
        "Consolas")?;
    hdc.SelectObjectFont(hfont)?;
    hdc.TextOut(10, 10, reading[0])?;
    hfont.DeleteObject()?;

    let hfont = HFONT::CreateFont(
        SIZE::new(0, 100),
        0,
        0,
        co::FW::DONTCARE,
        false,
        false,
        false,
        co::CHARSET::DEFAULT,
        co::OUT_PRECIS::DEFAULT,
        co::CLIP::DEFAULT_PRECIS,
        co::QUALITY::DEFAULT,
        co::PITCH::DEFAULT,
        "Consolas")?;
    hdc.SelectObjectFont(hfont)?;
    hdc.TextOut(10 + hdc.GetTextExtentPoint32(reading[0])?.cx, 10, reading[1])?;
    hfont.DeleteObject()?;

    hwnd.EndPaint(&ps);
    Ok(())
}

// fn select_units(&check) -> SysResult<()> {
//     options_submenu.CheckMenuRadioItem(
//         IdPos::Id(ids::OPTIONS_KG),
//         IdPos::Id(ids::OPTIONS_N),
//         IdPos::Id(check),
//     )?;

//     Ok(())
// }