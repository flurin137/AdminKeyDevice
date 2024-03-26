mod serial_wrapper;

use crate::serial_wrapper::SerialWrapper;
use serial2::SerialPort;
use std::{io, path::PathBuf};

fn main() {
    match write_message("fuuuu") {
        Ok(_) => println!("Sucessfully written password to device"),
        Err(err) => println!("Error writing to device: {err}"),
    }
}

fn write_message(message: &str) -> io::Result<()> {
    let available_ports = SerialPort::available_ports()?;
    let matching_device = available_ports
        .iter()
        .find(|a| is_correct_device(*a))
        .ok_or(std::io::Error::new(
            io::ErrorKind::NotFound,
            "No matching device found",
        ))?;

    let port = SerialPort::open(matching_device, 115200)?;

    loop {
        let bytes = message.as_bytes();
        port.write(&bytes)?;

        let mut buffer = [0; 256];
        port.read(&mut buffer)?;
    }
}

fn is_correct_device(device: &PathBuf) -> bool {
    println!("Opening Port {:?}", device);

    let port = match SerialWrapper::new(device) {
        Ok(port) => port,
        _ => return false,
    };

    if let Err(_) = port.write("Saaaay Whaaat") {
        return false;
    }

    let message = match port.read() {
        Ok(message) => message,
        Err(_) => return false,
    };

    println!("Device returned message {message}");

    return message == "Fuck YOU";
}
