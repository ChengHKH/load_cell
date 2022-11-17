// #![windows_subsystem = "windows"]

pub mod app;
pub mod ids;
pub mod ui;

use app::App;
use winsafe::{prelude::*, co, AnyResult, HWND};

fn main() {
    println!("Finding serial ports...");
    app::list_ports();
    if let Err(e) = run_app() {
        HWND::NULL.MessageBox(&e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
    }
}

fn run_app() -> AnyResult<i32> {
    App::new()
        .run()
        .map_err(|err| err.into())
}