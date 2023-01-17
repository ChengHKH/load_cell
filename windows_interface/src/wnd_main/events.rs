use winsafe::{prelude::*,};

use crate::dlg;
use crate::wnd_modal::WndModal;

use super::{WndMain, ids, funcs};

impl WndMain {
    pub(super) fn events(&self) {
        #[cfg(not(test))]
        self.window.on().wm_create({
            let main_window = self.window.clone(); 
            move |_| {
                let ports = funcs::get_ports();
                
                let port = match ports {
                    Some(info) => {
                        let (names, port_map) = funcs::get_port_names(info);
                        let selection = dlg::dlg_select_port(&main_window, names).unwrap();
                        port_map.get(&selection)
                    },
                    None => {
                        dlg::dlg_no_ports(&main_window);
                        return Ok(0)
                    },
                };
    
                let data = match port {
                    Some(p) => funcs::open_port(p),
                    None => {
                        dlg::dlg_not_connected(&main_window);
                        return Ok(0);
                    },
                };
                
                Ok(0)
           }
        });
    
        #[cfg(test)]
        self.window.on().wm_create({
            let main_window = self.window.clone(); 
            move |_| {
                let names = vec!["Test".to_owned(), "Test".to_owned()];
                dlg::dlg_select_port(&main_window, names);
                Ok(0)
           }
        });
    
        self.window.on().wm_command_accel_menu(ids::FILE_NEW, {
            let main_window = self.window.clone(); 
            move || {
                let logger = WndModal::new(&main_window);
                Ok(())
            }
        });
    
    }
}