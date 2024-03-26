mod serial_wrapper;

use crate::serial_wrapper::SerialWrapper;
use serial2::SerialPort;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let matching_device = get_matching_device().ok_or("Unable to find device".to_owned())?;

    matching_device
        .write("PASSWORD")
        .map_err(|e| format!("{e}"))
}

fn get_matching_device() -> Option<SerialWrapper> {
    let available_ports = SerialPort::available_ports().ok()?;
    let matching_device = available_ports.iter().find(|a| is_correct_device(*a))?;
    SerialWrapper::new(matching_device).ok()
}

fn is_correct_device(device: &PathBuf) -> bool {
    println!("Checking Port {:?}", device);

    let port = match SerialWrapper::new(device) {
        Ok(port) => port,
        _ => return false,
    };

    if let Err(_) = port.write("Whaaat") {
        return false;
    }

    let message = match port.read() {
        Ok(message) => message,
        Err(_) => return false,
    };
    
    return message == "Fuck YOU";
}
