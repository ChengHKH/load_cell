use std::collections::HashMap;
use serialport::{SerialPort, SerialPortType, SerialPortInfo};

fn get_reading<'a>() -> [&'a str; 2] {
    let value = "1234";

    let unit = " g";

    [value, unit]
}

pub fn get_ports() -> Option<Vec<SerialPortInfo>> {
    println!("Finding serial ports...");
    let ports = serialport::available_ports().expect("No ports found!");
    let mut arduino_ports = Vec::new();
    
    for p in ports {
        if let SerialPortType::UsbPort(ref i) = p.port_type {
            if i.vid == 9025 || i.vid == 10755 {
                arduino_ports.push(p);
            }
        }
    };

    if arduino_ports.len() == 0 {
        None
    } else {
        Some(arduino_ports)
    }
}

pub fn get_port_names(info: Vec<SerialPortInfo>) -> (Vec<String>, HashMap<String, String>) {
    let mut usb_names = Vec::new();
    let mut port_map = HashMap::new();
    for i in info {
        if let SerialPortType::UsbPort(usb) = i.port_type {
            let name = usb.product.unwrap_or_else(
                || usb.manufacturer.unwrap_or_else(
                    || String::from("Unknown")) + "USB");

            usb_names.push(name.clone());
            port_map.insert(name, i.port_name);
        }
    };
    (usb_names, port_map)
}

pub fn open_port(port_name: String) -> serialport::Result<Box<dyn SerialPort>> {
        serialport::new(port_name, 115200).open()
}