#![windows_subsystem = "windows"]

#[cfg(not(target_arch = "wasm32"))]

mod app;
use app::WindowsInterface;

fn main() {
    env_logger::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Load Cell Reader",
        native_options,
        Box::new(|cc| Box::new(WindowsInterface::new(cc)))
    );
}