use winsafe::{prelude::*, co, gui, SIZE, POINT, HWND, HFONT, MulDiv};

use super::Reader;

pub(super) fn build(parent: &impl GuiParent) -> Reader {
    let window = gui::WindowControl::new(
        parent,
        gui::WindowControlOpts {
            position: POINT::new(0, 0),
            size: SIZE::new(360, 130),
            ..Default::default()
        }
    );

    Reader {window}
}

pub(super) fn draw_reading(hwnd: &HWND, reading: [&str; 2]) -> winsafe::AnyResult<()> {
    let hdc = hwnd.BeginPaint()?;
    
    let hfont = HFONT::CreateFont(
        SIZE::new(0, -MulDiv(75, hdc.GetDeviceCaps(co::GDC::LOGPIXELSY), 72)),
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
    hdc.SelectObject(&hfont)?;
    hdc.TextOut(10, 10, reading[0])?;
    hfont.DeleteObject()?;

    let hfont = HFONT::CreateFont(
        SIZE::new(0, -MulDiv(75, hdc.GetDeviceCaps(co::GDC::LOGPIXELSY), 72)),
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
    hdc.SelectObject(&hfont)?;
    hdc.TextOut(10 + hdc.GetTextExtentPoint32(reading[0])?.cx, 10, reading[1])?;
    hfont.DeleteObject()?;

    Ok(())
}