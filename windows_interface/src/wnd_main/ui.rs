use winsafe::{prelude::*, co, gui, SIZE};

use super::{WndMain, ids};
use super::Reader;

pub(super) fn build() -> WndMain {
    let menu = build_main_menu().unwrap();
    
    let window = gui::WindowMain::new(
        gui::WindowMainOpts {
            title: "Load Cell Reader".to_owned(),
            class_icon: gui::Icon::Id(101),
            size: SIZE::new(360, 130),
            menu,
            ..Default::default()
        }
    );

    let reader = Reader::new(&window);

    WndMain {window, reader}
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
        winsafe::MenuEnum::Submenu(&file_submenu, "&File"),
        winsafe::MenuEnum::Submenu(&options_submenu, "&Options"),
    ])?;

    Ok(main_menu)
}

// fn select_units(&check) -> SysResult<()> {
//     options_submenu.CheckMenuRadioItem(
//         IdPos::Id(ids::OPTIONS_KG),
//         IdPos::Id(ids::OPTIONS_N),
//         IdPos::Id(check),
//     )?;

//     Ok(())
// }
