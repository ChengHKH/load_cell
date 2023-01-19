use std::cell::RefCell;
use std::rc::Rc;
use winsafe::{prelude::*, co, gui, msg, HBRUSH, HFONT, POINT, SIZE, MulDiv};

#[derive(Clone)]
pub struct Buttons {
    window: gui::WindowControl,
    pub button_one: Option<gui::Button>,
    pub button_two: Option<gui::Button>,
    pub button_three: Option<gui::Button>,
}

impl Buttons {
    pub fn new(
        parent: &impl GuiParent,
        size: SIZE,
        text_one: Option<&str>,
        text_two: Option<&str>,
        text_three: Option<&str>
    ) -> Buttons {
        let mut position = [size.cx - 81, size.cx - 159, size.cx - 237].into_iter();

        let window = gui::WindowControl::new(
            parent,
            gui::WindowControlOpts {
                class_bg_brush: gui::Brush::Color(co::COLOR::MENU),
                position: POINT::new(0, size.cy - 41),
                size: SIZE::new(size.cx, 41),
                ..Default::default()
            }
        );

        let button_three = match text_one {
            Some(text) => {
                Some(gui::Button::new(
                    parent,
                    gui::ButtonOpts {
                        text: text_three.unwrap().to_owned(),
                        position: POINT::new(position.next().unwrap(), 118),
                        width: 72,
                        height: 23,
                        ..Default::default()
                    }
                ))
            },
            None => None,
        };

        let button_two = match text_one {
            Some(text) => {
                Some(gui::Button::new(
                    parent,
                    gui::ButtonOpts {
                        text: text_two.unwrap().to_owned(),
                        position: POINT::new(position.next().unwrap(), 118),
                        width: 72,
                        height: 23,
                        ..Default::default()
                    }
                ))
            },
            None => None,
        };

        let button_one = match text_one {
            Some(text) => {
                Some(gui::Button::new(
                    parent,
                    gui::ButtonOpts {
                        text: text_one.unwrap().to_owned(),
                        position: POINT::new(position.next().unwrap(), size.cy - 32),
                        width: 72,
                        height: 23,
                        ..Default::default()
                    }
                ))
            },
            None => {
                Some(gui::Button::new(
                    parent,
                    gui::ButtonOpts {
                        text: "OK".to_owned(),
                        position: POINT::new(position.next().unwrap(), size.cy - 32),
                        width: 72,
                        height: 23,
                        ..Default::default()
                    }
                ))
            },
        };

        let new_self = Buttons {window, button_one, button_two, button_three};
        new_self
    }
}

#[derive(Clone)]
struct Dlg {
    pub window: gui::WindowModal,
    main_instruction: gui::Label,
    secondary_instruction: gui::Label,
    buttons: Buttons,
}

impl Dlg {
    fn new(
        parent: &impl GuiParent,
        title: &str,
        header: &str,
        body: &str,
        button_text_one: Option<&str>,
        button_text_two: Option<&str>,
        button_text_three: Option<&str>,
        size: SIZE,
    ) -> Dlg {
        let window = gui::WindowModal::new(
            parent,
            gui::WindowModalOpts {
                class_bg_brush: gui::Brush::Color(co::COLOR::WINDOW),
                title: title.to_owned(),
                size,
                ..Default::default()
            }
        );
    
        let main_instruction = gui::Label::new(
            &window,
            gui::LabelOpts {
                text: header.to_owned(),
                position: POINT::new(10, 10),
                size: SIZE::new(330, 30),
                ..Default::default()
            }
        );
    
        let secondary_instruction = gui::Label::new(
            &window,
            gui::LabelOpts {
                text: body.to_owned(),
                position: POINT::new(10, 50),
                size: SIZE::new(330, 20),
                ..Default::default()
            }
        );
    
        let buttons = Buttons::new(&window, size, button_text_one, button_text_two, button_text_three);
        
        let new_self = Dlg {window, main_instruction, secondary_instruction, buttons};
        new_self.font();
        new_self
    }

    fn font(&self) {
        self.window.on().wm_create({
            let main_instruction = self.main_instruction.clone();
            move |_| {
                let hwnd = main_instruction.hwnd();
                let hdc = hwnd.GetWindowDC()?;
                let font = HFONT::CreateFont(
                    SIZE::new(0, -MulDiv(12, hdc.GetDeviceCaps(co::GDC::LOGPIXELSY), 72)),
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
                    "Segoe UI")?;
                hwnd.SendMessage(msg::wm::SetFont {
                    hfont: font,
                    redraw: true,
                });
                hwnd.ReleaseDC(hdc)?;
                Ok(0)
            }
        });

        self.window.on().wm_ctl_color_static({
            let main_instruction = self.main_instruction.clone();
            move |m: msg::wm::CtlColorStatic| {
                if m.hwnd == main_instruction.hwnd() {
                    m.hdc.SetTextColor(winsafe::COLORREF::new(0x00, 0x33, 0x99))?;
                }
                m.hdc.SetBkMode(co::BKMODE::TRANSPARENT)?;
                let color = HBRUSH::GetSysColorBrush(co::COLOR::WINDOW)?;
                Ok(color)
            }
        });

        self.window.on().wm_destroy({
            let main_instruction = self.main_instruction.clone();
            move || {
                let font = main_instruction.hwnd().SendMessage(msg::wm::GetFont {}).unwrap();
                font.DeleteObject()?;
                Ok(())
            }
        });
    }

    fn build(self) -> (gui::WindowModal, Buttons) {
            (self.window, self.buttons)
    }
}

#[derive(Clone)]
pub struct DlgDropDown {
    pub window: gui::WindowModal,
    pub buttons: Buttons,
    pub drop_down: gui::ComboBox,

    pub return_value: Rc<RefCell<Option<String>>>,
}

impl DlgDropDown {
    pub fn new(
        parent: &impl GuiParent,
        title: &str,
        header: &str,
        body: &str,
        list: Vec<String>,
        button_text_one: Option<&str>,
    ) -> DlgDropDown {
        let (window, buttons) = Dlg::new(
            parent,
            title,
            header,
            body,
            button_text_one,
            Some("Cancel"),
            None,
            SIZE::new(350, 150)
        ).build();

        let drop_down = gui::ComboBox::new(
            &window,
            gui::ComboBoxOpts {
                position: POINT::new(10, 50),
                items: list,
                ..Default::default()
            }
        );
        
        let new_self = DlgDropDown {
            window,
            buttons,
            drop_down,
            return_value: Rc::new(RefCell::new(None)),
        };

        new_self.events();
        new_self
    }

    pub fn show(&self) -> Option<String> {
        self.window.show_modal();
        self.return_value.borrow().as_ref().map(|s| s.clone())
    }
}