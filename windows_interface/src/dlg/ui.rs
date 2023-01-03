use winsafe::{prelude::*, co, gui, msg, POINT, SIZE};

enum Buttons {
    One(gui::Button),
    Two(gui::Button, gui::Button),
    Three(gui::Button, gui::Button, gui::Button)
}

impl Buttons {
    pub fn new(parent: &impl GuiParent, one: Option<&str>, two: Option<&str>, three: Option<&str>) -> Buttons {
        match one {
            Some(text) => {
                gui::Button::new(
                    parent,
                    gui::ButtonOpts {
                        text: "Connect".to_owned(),
                        position: POINT::new(269, 118),
                        width: 72,
                        height: 23,
                        ..Default::default()
                    }
                )
            },
            None => ,
        };
    }
}

struct CustomDlg {
    window: gui::WindowModal,
    main_instruction: gui::Label,
    secondary_instruction: gui::Label,
    buttons: Buttons,
}

impl CustomDlg {
    pub fn new(parent: &impl GuiParent) -> CustomDlg {
        let window = gui::WindowModal::new(
            parent,
            gui::WindowModalOpts {
                class_bg_brush: gui::Brush::Color(co::COLOR::WINDOW),
                title: "Select port".to_owned(),
                size: SIZE::new(350, 150),
                ..Default::default()
            }
        );
    
        let main_instruction = gui::Label::new(
            &window,
            gui::LabelOpts {
                text: "Connect to an Arduino".to_owned(),
                position: POINT::new(10, 10),
                size: SIZE::new(330, 30),
                ..Default::default()
            }
        );
    
        let secondary_instruction = gui::Label::new(
            &window,
            gui::LabelOpts {
                text: "Please select an Arduino to connect to.".to_owned(),
                position: POINT::new(10, 50),
                size: SIZE::new(330, 20),
                ..Default::default()
            }
        );
    
        let buttons = ;
    
        let new_self = Dlg {window, main_instruction, secondary_instruction, buttons};
        new_self.font();
        new_self
    }

    fn font(&self) {
        self.window.on().wm_create({
            let main_instruction = self.main_instruction.clone();
            move |_| {
                let hwnd = main_instruction.hwnd();
                ui::draw_instruction_main_font(hwnd)?;
                Ok(0)
            }
        });

        self.window.on().wm_ctl_color_static({
            let main_instruction = self.main_instruction.clone();
            move |m: msg::wm::CtlColorStatic| {
                if m.hwnd == main_instruction.hwnd() {
                    ui::draw_instruction_main_color(m.hdc)?;
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
}

impl DlgSelectPort {
    pub fn build_select_port(parent: &impl GuiParent) -> gui::WindowModal {
        
    }
    
    pub fn build_select_port_cancel(parent: &impl GuiParent, modal: &str, number: u8) -> gui::Button {
        gui::Button::new(
            parent,
            gui::ButtonOpts {
                text: "Cancel".to_owned(),
                position: btn_position(modal, number).unwrap(),
                width: 72,
                height: 23,
                ..Default::default()
            }
        )
    }
    
    pub fn build_select_port_connect(parent: &impl GuiParent, modal: &str, number: u8) -> gui::Button {
        
    }
    
    pub fn build_select_port_instruction_main(parent: &impl GuiParent) -> gui::Label {
        
    }
    
    pub fn build_select_port_instruction_secondary(parent: &impl GuiParent) -> gui::Label {
        
    }
    
    pub fn build_select_port_list(parent: &impl GuiParent, names: Vec<String>) -> gui::ComboBox {
        gui::ComboBox::new(
            parent,
            gui::ComboBoxOpts {
                position: POINT::new(10, 50),
                items: names,
                ..Default::default()
            }
        )
    }
    
    
    pub fn draw_instruction_main_color(hdc: HDC) -> AnyResult<()> {
        hdc.SetTextColor(winsafe::COLORREF::new(0x00, 0x33, 0x99))?;
        Ok(())
    }
    
    pub fn draw_instruction_main_font(hwnd: HWND) -> AnyResult<()> {
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
        Ok(())
    }

    fn btn_position(modal: &str, number: u8) -> Option<POINT> {
        match modal {
            "input" => match number {
                1 => Some(POINT::new(269, 118)),
                2 => Some(POINT::new(191, 118)),
                _ => None,
            }
            "error" => match number {
                1 => Some(POINT::new(269, 118)),
                2 => Some(POINT::new(191, 118)),
                _ => None,
            },
            _ => None,
        }
    }
}