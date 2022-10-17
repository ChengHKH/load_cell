pub mod app;
pub mod ui;

use app::App;
use native_windows_gui as nwg;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = App::
}