use serial2::SerialPort;

use crate::serial_wrapper::SerialWrapper;

pub struct AdminKey {
    serial_wrapper: SerialWrapper,
}

impl AdminKey {
    pub fn connect() -> Result<Self, String> {
        let serial_wrapper = get_serial_port().ok_or("err")?;

        println!("Found Device {}", serial_wrapper);

        Ok(Self { serial_wrapper })
    }

    pub fn write(&self, message: &str) -> Result<(), String> {
        self.serial_wrapper
            .write(message)
            .map_err(|e| format!("{e}"))
    }
}

fn get_serial_port() -> Option<SerialWrapper> {
    let available_ports = SerialPort::available_ports().ok()?;

    for port in available_ports {
        println!("Checking Port {:?}", port);

        let port = match SerialWrapper::new(port.to_owned()) {
            Ok(port) => port,
            _ => continue,
        };

        if port.write("Whaaat").is_err() {
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
