mod serial_wrapper;

use crate::serial_wrapper::SerialWrapper;
use serial2::SerialPort;

fn main() -> Result<(), String> {
    let matching_device = get_matching_device().ok_or("Unable to find device".to_owned())?;

    println!("Found Device {}", matching_device);

    matching_device
        .write("FUCK YOU")
        .map_err(|e| format!("{e}"))
}

fn get_matching_device() -> Option<SerialWrapper> {
    let available_ports = SerialPort::available_ports().ok()?;

    for port in available_ports {
        println!("Checking Port {:?}", port);

        let port = match SerialWrapper::new(port.to_owned()) {
            Ok(port) => port,
            _ => continue,
        };

        if let Err(_) = port.write("Whaaat") {
            continue;
        }

        let message = match port.read() {
            Ok(message) => message,
            Err(_) => continue,
        };

        if message == "Fuck YOU" {
            return Some(port);
        }
    }

    None
}
