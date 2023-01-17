#![windows_subsystem = "windows"]

use winsafe::{prelude::*, co, AnyResult, HWND};

mod wnd_main;
mod wnd_modal;
mod dlg;
mod reader;
mod logger;

use wnd_main::WndMain;

fn main() {
    if let Err(e) = run_app() {
        HWND::NULL.MessageBox(&e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
    }
}

fn run_app() -> AnyResult<i32> {
    WndMain::new()
        .run()
        .map_err(|err| err.into())
}

#[cfg(test)]
#[test]
fn test() {
    main()
}