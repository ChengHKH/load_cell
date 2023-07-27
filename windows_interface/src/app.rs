use core::fmt;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug, PartialEq)]

enum Units {
    Gram,
    Kilogram,
    Newton,
    Pound,
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Units::Gram => write!(f, "g"),
            Units::Kilogram => write!(f, "kg"),
            Units::Newton => write!(f, "N"),
            Units::Pound => write!(f, "pb"),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct  WindowsInterface {
    value: f32,
    unit: Units,
}

impl Default for WindowsInterface {
    fn default() -> Self {
        Self {
            value: 0.0,
            unit: Units::Gram,
        }
    }
}

impl WindowsInterface {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for WindowsInterface {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {value , unit} = self;
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");

        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(label);
        //     });

        //     ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        //     if ui.button("Increment").clicked() {
        //         *value += 1.0;
        //     }

        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         ui.horizontal(|ui| {
        //             ui.spacing_mut().item_spacing.x = 0.0;
        //             ui.label("powered by ");
        //             ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //             ui.label(" and ");
        //             ui.hyperlink_to(
        //                 "eframe",
        //                 "https://github.com/emilk/egui/tree/master/crates/eframe",
        //             );
        //             ui.label(".");
        //         });
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(format!("{}", value));
                egui::ComboBox::from_id_source("unit-select")
                    .selected_text(format!("{}", unit))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(30.0);
                        ui.selectable_value(unit, Units::Gram, "g");
                        ui.selectable_value(unit, Units::Kilogram, "kg");
                        ui.selectable_value(unit, Units::Newton, "N");
                        ui.selectable_value(unit, Units::Pound, "lb");
                    });
            });
            egui::warn_if_debug_build(ui);
        });
    }
}