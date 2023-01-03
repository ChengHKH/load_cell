use serialport::{SerialPortType, SerialPortInfo};

fn get_reading<'a>() -> [&'a str; 2] {
    let value = "1234";

    let unit = " g";

    [value, unit]
}

pub fn get_ports() -> Option<Vec<serialport::SerialPortInfo>> {
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

pub fn get_port_names(info: Vec<SerialPortInfo>) -> Vec<String> {
    let mut port_names = Vec::new();
    for i in info {
        if let serialport::SerialPortType::UsbPort(usb) = i.port_type {
            let name = usb.product.unwrap_or_else(
                || usb.manufacturer.unwrap_or_else(
                    || String::from("Unknown")) + "USB");

            port_names.push(name);
        }
    };
    port_names
}

pub fn open_port(info: SerialPortInfo) -> serialport::Result<Box<dyn serialport::SerialPort>> {
        serialport::new(info.port_name, 115200).open()
}